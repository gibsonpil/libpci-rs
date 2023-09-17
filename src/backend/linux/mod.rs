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

use crate::backend::common::PciDevice;
use std::fs::*;

use super::common::*;

// ahaha this particular code is by Shibe Drill

#[inline]
fn comps_from_linux_pci_addr(address: &str) -> Result<(u32, u8, u8, u8), ()> {
    let comps_vec: Vec<&str> = address.split(|char| (char == ':') | (char == '.')).collect();
    if comps_vec.len() != 4 {
        return Err(());
    }
    Ok((
        u32::from_str_radix(comps_vec[0], 16).unwrap(),
        u8::from_str_radix(comps_vec[1], 16).unwrap(),
        u8::from_str_radix(comps_vec[2], 16).unwrap(),
        u8::from_str_radix(comps_vec[3], 16).unwrap(),
    ))
}

#[inline]
pub fn _get_pci_list() -> Result<Vec<PciDevice>, PciEnumerationError> {
    let mut device_list: Vec<PciDevice> = Vec::new();

    /*
    On Linux, PCI device information is stored in /sys/bus/pci/devices/.
    In this directory, there are multiple directories named after PCI addresses in the form of
    0000:00:00.0 where each 0 can be a valid hex digit. These directories contain files that
    hold the information needed to populate the PCI device structure. As follows is the list
    of files and the fields they populate:
        Label:
        Domain: First 4 digits of the address.
        Bus: Second set of digits, 2 digits long.
        Device: 3rd set of digits, 2 digits long.
        Function: Final digit.
        Vendor ID: file 'vendor', 0x prefix
        Device ID: file 'device', 0x prefix
        Subsys Vendor ID: file 'subsystem_device', 0x prefix
        Subsys Device ID: file 'subsystem_vendor', 0x prefix
        Device Class: file 'class', 0x prefix
        Revision ID: file 'revision', 0x prefix
    */

    for directory in read_dir("/sys/bus/pci/devices/").unwrap() {
        let label = String::from("Label");
        let vendor_id = get_pci_device_attribute_u16(&directory, "vendor")?; // Vendor ID
        let device_id = get_pci_device_attribute_u16(&directory, "device")?; // Device ID
        let subsys_device_id = get_pci_device_attribute_u16(&directory, "subsystem_device")?; // Subsystem Device ID
        let subsys_vendor_id = get_pci_device_attribute_u16(&directory, "subsystem_vendor")?; // Subsystem Vendor ID

        let class_code = get_pci_device_attribute_u32(&directory, "class")?;

        let class: u8 = ((class_code >> 16) & 0xFF) as u8; // Device Class
        let subclass: u8 = ((class_code >> 8) & 0xFF) as u8; // Device Subclass
        let programming_interface: u8 = (class_code & 0xFF) as u8; // Device Programming Interface

        let revision_id = get_pci_device_attribute_u8(&directory, "revision")?; // Revision ID
        let components = comps_from_linux_pci_addr(&directory.unwrap().file_name().to_str().unwrap()).unwrap(); // TODO: handle in case of error as to not panic on unwrap.
        let (domain, bus, device, function) = components;

        device_list.push(PciDevice {
            domain,
            bus,
            device,
            function,
            label,
            vendor_id,
            device_id,
            subsys_device_id,
            subsys_vendor_id,
            class,
            subclass,
            programming_interface,
            revision_id,
        })
    }

    // return the list at the end once all the devices are in it.
    Ok(device_list)
}

#[inline]
pub fn _get_pci_by_id(vendor: u16, device: u16) -> Result<PciDevice, PciEnumerationError> {
    todo!()
}

