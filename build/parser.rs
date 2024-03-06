// Copyright (c) 2024 Gibson Pilconis. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors
// may be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use std::num::ParseIntError;
use std::path::Path;
use phf_codegen::Map;
use proc_macro2::TokenStream;
use quote::quote;

use crate::types::*;

// A lot of inspiration for the overall architecture of this script
// (i.e. using PHF with quote) was taken from here: 
// https://github.com/lienching/pci-ids.rs/blob/main/build.rs

fn id<T, F>(input: &str, position: usize, radix_function: F) -> T
where F: Fn(&str, u32) -> Result<T, ParseIntError> {
    let id = input.split(' ').collect::<Vec<&str>>()[position];
    radix_function(id, 16).unwrap()
}

fn name(input: &str) -> &str {
    // pci.ids puts two spaces before the name.
    input.split("  ").last().unwrap()
}

fn clean(input: &str) -> String {
    // Filter out tabs as we don't need them to parse data.
    input.replace('\t', "")
}

pub fn get_level(input: &str) -> usize {
    let indices: Vec<_> = input.match_indices('\t').collect();
    indices.len()
}

pub fn vendor(input: &str) -> PciVendorEntry {
    let cleaned = clean(input);
    let id = id(cleaned.as_str(), 0, u16::from_str_radix);
    let name = name(cleaned.as_str()).to_string();

    PciVendorEntry {
        id,
        name,
        devices: vec![]
    }
}

pub fn device(input: &str) -> PciDeviceEntry {
    let cleaned = clean(input);
    let id = id(cleaned.as_str(), 0, u16::from_str_radix);
    let name = name(cleaned.as_str()).to_string();

    PciDeviceEntry {
        id,
        name,
        subsystems: vec![]
    }
}

pub fn subsystem(input: &str) -> PciSubsystemEntry {
    let cleaned = clean(input);
    let subvendor = id(cleaned.as_str(), 0, u16::from_str_radix);
    let subdevice = id(cleaned.as_str(), 1, u16::from_str_radix);
    let name = name(cleaned.as_str()).to_string();

    PciSubsystemEntry {
        subvendor,
        subdevice,
        name
    }
}

pub fn ingest_class_database(data: &str) -> Map<u8> {
    todo!()
}

pub fn ingest_pci_database(data: &str) -> Map<u16> {
    let mut result = Map::new();

    let mut current_vendor: Option<PciVendorEntry> = None;
    let mut current_device: Option<PciDeviceEntry> = None;
    let mut current_level: usize; // 0 - vendor, 1 - device, 2 - subdevice

    let i = 0;

    for entry in data.split('\n').filter(|x| {!x.is_empty()}) {
        // Assess our position.
        current_level = get_level(entry);

        if current_level == 0 {
            if let Some(vendor) = current_vendor.take() {
                result.entry(vendor.id, &quote!(#vendor).to_string());
            }
            current_vendor = Some(vendor(entry));
        } else if current_level == 1 {
            if let Some(device) = current_device.take() {
                current_vendor
                    .as_mut()
                    .unwrap()
                    .devices
                    .push(device);
            }
            current_device = Some(device(entry));
        } else if current_level == 2 {
            current_device
                .as_mut()
                .unwrap()
                .subsystems
                .push(subsystem(entry));
        }
    }

    if let Some(vendor) = current_vendor.take() {
        result.entry(vendor.id, &quote!(#vendor).to_string());
    }

    result
}

impl quote::ToTokens for PciVendorEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let PciVendorEntry {
            id,
            name,
            devices
        } = self;
        
        let devices = devices.iter().map(|PciDeviceEntry { id, name, subsystems }| {
            quote! {
                PciDeviceEntry { id: #id, name: #name, subsystems: &[#(#subsystems),*] }
            }
        });
        
        tokens.extend(quote! {
            PciVendorEntry { id: #id, name: #name, devices: &[#(#devices),*] }
        });
    }
}

impl quote::ToTokens for PciSubsystemEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let PciSubsystemEntry {
            subvendor,
            subdevice,
            name
        } = self;
        
        tokens.extend(quote! {
            PciSubsystemEntry { subvendor: #subvendor, subdevice: #subdevice, name: #name }
        });
    }
}

pub fn ingest_pciids(path: &Path) -> PciIdsParsed {
    let pciids_raw = fs::read_to_string(path).unwrap();
    let pciids_filtered: Vec<&str> =
        pciids_raw
            .split("\r\n") // Can we please drop carriage returns already?
            .filter(|str| !str.contains('#')) // Filter comments.
            .filter(|str| !str.is_empty())
            .collect();

    let mut pci_database_raw: Vec<&str> = vec![];
    let mut pci_classes_raw: Vec<&str> = vec![];

    // TODO: make this more compact.
    // Separate device entries from class entries.
    let mut collecting_classes = false;
    for line in pciids_filtered {
        // Once we start getting class entries we should stash them into pci_classes_raw.
        if line.starts_with("C ") {
            collecting_classes = true;
        }
        if collecting_classes {
            pci_classes_raw.push(line);
        } else {
            pci_database_raw.push(line);
        }
    }

    // let pci_classes = ingest_class_database(pci_classes_raw.join("\n").as_str());

    PciIdsParsed {
        pci: Some(ingest_pci_database(pci_database_raw.join("\n").as_str())),
        class: None
    }
}
