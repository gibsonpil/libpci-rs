// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

mod cxx;
mod parser;
pub mod types;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::parser::ingest_pciids;

#[allow(unused_imports)]
use crate::cxx::build_cxx_module;

fn generate_phf_data() {
    let devices_path = Path::new(&env::var("OUT_DIR").unwrap()).join("pci_devices_phf.rs");
    let classes_path = Path::new(&env::var("OUT_DIR").unwrap()).join("pci_classes_phf.rs");

    let mut devices_file = BufWriter::new(File::create(devices_path).unwrap());
    let mut classes_file = BufWriter::new(File::create(classes_path).unwrap());

    let pci_ids_parsed = ingest_pciids(Path::new("pciids/pci.ids"));

    writeln!(
        devices_file,
        "static VENDORS: phf::Map<u16, PciVendorEntry> = {};",
        &pci_ids_parsed.pci.build()
    )
    .expect("failed to write VENDORS to registry!");

    writeln!(
        classes_file,
        "static CLASSES: phf::Map<u8, PciClassEntry> = {};",
        &pci_ids_parsed.class.build()
    )
    .expect("failed to write CLASSES to registry!");

    println!("cargo:rerun-if-changed=pciids/pci.ids");
}

fn main() {
    cfg_if::cfg_if! {
        // Add targets with backends written entirely in Rust to this list.
        if #[cfg(not(any(target_os = "windows", target_os = "linux")))] {
            build_cxx_module();
        }
    }

    #[cfg(feature = "pciids")]
    generate_phf_data();
}
