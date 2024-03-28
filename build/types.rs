// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

use phf_codegen::Map;
use proc_macro2::TokenStream;
use quote::quote;

pub struct PciVendorEntry {
    pub(crate) id: u16,
    pub(crate) name: String,
    pub(crate) devices: Vec<PciDeviceEntry>,
}

pub struct PciDeviceEntry {
    pub(crate) id: u16,
    pub(crate) name: String,
    pub(crate) subsystems: Vec<PciSubsystemEntry>,
}

pub struct PciSubsystemEntry {
    pub(crate) subvendor: u16,
    pub(crate) subdevice: u16,
    pub(crate) name: String,
}

pub struct PciClassEntry {
    pub(crate) id: u8,
    pub(crate) name: String,
    pub(crate) subclasses: Vec<PciSubclassEntry>,
}

pub struct PciSubclassEntry {
    pub(crate) id: u8,
    pub(crate) name: String,
    pub(crate) progs: Vec<PciProgEntry>,
}

pub struct PciProgEntry {
    pub(crate) id: u8,
    pub(crate) name: String,
}

pub struct PciIdsParsed {
    pub(crate) pci: Map<u16>,
    pub(crate) class: Map<u8>,
}

impl quote::ToTokens for PciVendorEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let PciVendorEntry { id, name, devices } = self;

        let devices = devices.iter().map(
            |PciDeviceEntry {
                 id,
                 name,
                 subsystems,
             }| {
                quote! {
                    PciDeviceEntry { id: #id, name: #name, subsystems: &[#(#subsystems),*] }
                }
            },
        );

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
            name,
        } = self;

        tokens.extend(quote! {
            PciSubsystemEntry { subvendor: #subvendor, subdevice: #subdevice, name: #name }
        });
    }
}

impl quote::ToTokens for PciClassEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let PciClassEntry {
            id,
            name,
            subclasses,
        } = self;

        let subclasses = subclasses
            .iter()
            .map(|PciSubclassEntry { id, name, progs }| {
                quote! {
                    PciSubclassEntry { id: #id, name: #name, progs: &[#(#progs),*] }
                }
            });

        tokens.extend(quote! {
            PciClassEntry { id: #id, name: #name, subclasses: &[#(#subclasses),*] }
        });
    }
}

impl quote::ToTokens for PciProgEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let PciProgEntry { id, name } = self;

        tokens.extend(quote! {
            PciProgEntry { id: #id, name: #name }
        });
    }
}
