// Copyright (c) 2023 NamedNeon. All rights reserved.
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

// TODO: Implement ID lookup system

// Syntax: (as copied from the pci.ids file)
// vendor  vendor_name
//      device  device_name				<-- single tab
//		     subvendor subdevice  subsystem_name	<-- two tabs
// This tree syntax *might* be easy to parse. might not.
// To sanitize the database we need to get rid of all the lines with a # at the beginning, 
// ignoring any leading whitespace.
// First step, we find the lvl1 node with the vendor ID specified, and store the vendor name.
// Second step, we find the lvl2 node with the device ID specified, and store the device name.

use crate::backend::PciDevice;
use lazy_static::lazy_static;
use confindent::Confindent;

// This version of the data is dirty. We cannot use it.
static PCIIDS_DIRTY: &str = include_str!("../pciids/pci.ids");

lazy_static! {
    // The clean, structured version has to be constructed lazily. 
    // We can't do all this in a static.
    static ref PCIIDS: Confindent = PCIIDS_DIRTY    
        .lines()
        // Get rid of all comment lines
        .filter(|line| line.trim().starts_with("#"))
        .collect::<Vec<&str>>()
        .join("\n")
        .parse()
        .expect("Error while parsing PCIIDs database.");
}

pub fn vid_did_lookup(device: &PciDevice) -> String {
    let vendor = PCIIDS.child(format!("{:x}", device.vendor_id)).unwrap();
    let device = vendor.child(format!("{:x}", device.device_id)).unwrap();
    format!("{} {}", vendor.value().expect("Invalid vendor"), device.value().expect("Invalid device"))
}