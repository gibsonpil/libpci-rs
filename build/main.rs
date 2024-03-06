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


#![allow(unused_variables)]

pub mod types;
mod parser;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::parser::ingest_pciids;

// Syntax: (as copied from the pci.ids file)
// vendor  vendor_name
//      device  device_name				<-- single tab
//		     subvendor subdevice  subsystem_name	<-- two tabs
// This tree syntax *might* be easy to parse. might not.
// To sanitize the database we need to get rid of all the lines with a # at the beginning,
// ignoring any leading whitespace.
// First step, we find the lvl1 node with the vendor ID specified, and store the vendor name.
// Second step, we find the lvl2 node with the device ID specified, and store the device name.
fn generate_phf_data() {
    let devices_path = Path::new(&env::var("OUT_DIR").unwrap()).join("pci_devices_phf.rs");
    let classes_path = Path::new(&env::var("OUT_DIR").unwrap()).join("pci_classes_phf.rs");

    let mut devices_file = BufWriter::new(File::create(devices_path).unwrap());
    let classes_file = BufWriter::new(File::create(classes_path).unwrap());

    let pci_ids_parsed = ingest_pciids(Path::new("pciids/pci.ids"));

    writeln!(
        devices_file,
        "static VENDORS: phf::Map<u16, PciVendorEntry> = {};",
        pci_ids_parsed.pci.unwrap().build()
    ).expect("failed to write VENDORS to registry!");
    
    println!("cargo:rerun-if-changed=pciids/pci.ids");
}

fn main() {
    #[cfg(feature = "phf_data")]
    generate_phf_data();
}