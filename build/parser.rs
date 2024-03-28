// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

use crate::types::*;
use phf_codegen::Map;
use quote::quote;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        const LINE_BREAK: &str = "\r\n";
    } else {
        const LINE_BREAK: &str = "\n";
    }
}

// A lot of inspiration for the overall architecture of this helper
// (i.e. using PHF with quote) was taken from here:
// https://github.com/lienching/pci-ids.rs/blob/main/build.rs

fn id<T, F>(input: &str, position: usize, radix_function: F) -> T
where
    F: Fn(&str, u32) -> Result<T, ParseIntError>,
{
    let id = input.split(' ').collect::<Vec<&str>>()[position];
    radix_function(id, 16).unwrap()
}

fn name(input: &str) -> String {
    // pci.ids puts two spaces before the name.
    input.split("  ").last().unwrap().to_string()
}

fn clean(input: &str) -> String {
    // Filter out tabs as we don't need them to parse data.
    input.replace('\t', "")
}

fn get_level(input: &str) -> usize {
    let indices: Vec<_> = input.match_indices('\t').collect();
    indices.len()
}

fn try_level<T, F>(input: &str, level: usize, parser: F) -> Option<T>
where
    F: Fn(&str) -> T,
{
    if get_level(input) == level {
        return Some(parser(input));
    }
    None
}

pub fn vendor(input: &str) -> PciVendorEntry {
    let cleaned = clean(input);
    let id = id(cleaned.as_str(), 0, u16::from_str_radix);
    let name = name(cleaned.as_str());

    PciVendorEntry {
        id,
        name,
        devices: vec![],
    }
}

pub fn device(input: &str) -> PciDeviceEntry {
    let cleaned = clean(input);
    let id = id(cleaned.as_str(), 0, u16::from_str_radix);
    let name = name(cleaned.as_str());

    PciDeviceEntry {
        id,
        name,
        subsystems: vec![],
    }
}

pub fn subsystem(input: &str) -> PciSubsystemEntry {
    let cleaned = clean(input);
    let subvendor = id(cleaned.as_str(), 0, u16::from_str_radix);
    let subdevice = id(cleaned.as_str(), 1, u16::from_str_radix);
    let name = name(cleaned.as_str());

    PciSubsystemEntry {
        subvendor,
        subdevice,
        name,
    }
}

pub fn class(input: &str) -> PciClassEntry {
    let cleaned = clean(input);
    // ID is at position 1 due to "C" token
    let id: u8 = id(cleaned.as_str(), 1, u8::from_str_radix);
    let name = name(cleaned.as_str());

    PciClassEntry {
        id,
        name,
        subclasses: vec![],
    }
}

pub fn subclass(input: &str) -> PciSubclassEntry {
    let cleaned = clean(input);
    let id = id(cleaned.as_str(), 0, u8::from_str_radix);
    let name = name(cleaned.as_str());

    PciSubclassEntry {
        id,
        name,
        progs: vec![],
    }
}

pub fn prog(input: &str) -> PciProgEntry {
    let cleaned = clean(input);
    let id = id(cleaned.as_str(), 0, u8::from_str_radix);
    let name = name(cleaned.as_str());

    PciProgEntry { id, name }
}

pub fn ingest_pci_database(data: Vec<&str>) -> Map<u16> {
    let mut result = Map::new();

    let mut current_vendor: Option<PciVendorEntry> = None;
    let mut current_device: Option<PciDeviceEntry> = None;

    for entry in data {
        if let Some(value) = try_level(entry, 0, vendor) {
            if let Some(vendor) = current_vendor.take() {
                result.entry(vendor.id, &quote!(#vendor).to_string());
            }
            current_vendor = Some(value);
        } else if let Some(value) = try_level(entry, 1, device) {
            if let Some(device) = current_device.take() {
                current_vendor.as_mut().unwrap().devices.push(device);
            }
            current_device = Some(value);
        } else if let Some(value) = try_level(entry, 2, subsystem) {
            current_device.as_mut().unwrap().subsystems.push(value);
        }
    }

    if let Some(vendor) = current_vendor.take() {
        result.entry(vendor.id, &quote!(#vendor).to_string());
    }

    result
}

pub fn ingest_class_database(data: Vec<&str>) -> Map<u8> {
    let mut result = Map::new();

    let mut current_class: Option<PciClassEntry> = None;
    let mut current_subclass: Option<PciSubclassEntry> = None;

    for entry in data {
        if let Some(value) = try_level(entry, 0, class) {
            if let Some(class) = current_class.take() {
                result.entry(class.id, &quote!(#class).to_string());
            }
            current_class = Some(value);
        } else if let Some(value) = try_level(entry, 1, subclass) {
            if let Some(subclass) = current_subclass.take() {
                current_class.as_mut().unwrap().subclasses.push(subclass);
            }
            current_subclass = Some(value);
        } else if let Some(value) = try_level(entry, 2, prog) {
            current_subclass.as_mut().unwrap().progs.push(value);
        }
    }

    if let Some(class) = current_class.take() {
        result.entry(class.id, &quote!(#class).to_string());
    }

    result
}

pub fn ingest_pciids(path: &Path) -> PciIdsParsed {
    let pciids_raw = fs::read_to_string(path).unwrap();
    let pciids_filtered: Vec<&str> = pciids_raw
        .split(LINE_BREAK)
        .filter(|str| !clean(str).starts_with('#')) // Filter comments.
        .filter(|str| !str.is_empty())
        .collect();

    let split_idx = pciids_filtered
        .iter()
        .position(|x| x.starts_with("C "))
        .unwrap();

    let split = pciids_filtered.split_at(split_idx);

    let pci_database_raw = split.0.to_vec();
    let pci_classes_raw = split.1.to_vec();

    PciIdsParsed {
        pci: ingest_pci_database(pci_database_raw),
        class: ingest_class_database(pci_classes_raw),
    }
}
