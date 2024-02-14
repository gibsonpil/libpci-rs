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

macro_rules! get_pci_device_attribute {
    ($t:ty, $dir:expr, $attribute:expr) => {{
        let dir_usable = match $dir {
            Ok(f) => f,
            Err(_) => {
                return Err(PciEnumerationError::ReadDirectory);
            }
        };

        let file_contents = read_to_string(format!(
            "{}/{}",
            dir_usable.path().to_string_lossy(),
            $attribute
        ))?;
        let input_string = if let Some(stripped) = file_contents.strip_prefix("0x") {
            stripped
        } else {
            &file_contents
        }
        .trim();
        <$t>::from_str_radix(input_string, 16)
    }};
}

#[inline]
fn comps_from_linux_pci_addr(address: &str) -> Result<(u32, u8, u8, u8), PciEnumerationError> {
    let comps_vec: Vec<&str> = address
        .split(|char| (char == ':') | (char == '.'))
        .collect();
    // If this somehow fails, the OS is probably lying to us.
    if comps_vec.len() != 4 {
        return Err(PciEnumerationError::OsError);
    }
    Ok((
        u32::from_str_radix(comps_vec[0], 16)?,
        u8::from_str_radix(comps_vec[1], 16)?,
        u8::from_str_radix(comps_vec[2], 16)?,
        u8::from_str_radix(comps_vec[3], 16)?,
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
        Label: We don't actually know what this is.
        Domain: First 4 digits of the address.
        Bus: Second set of digits, 2 digits long.
        Device: 3rd set of digits, 2 digits long.
        Function: Final digit.
        Vendor ID: file 'vendor', 0x prefix
        Device ID: file 'device', 0x prefix
        Subsystem Vendor ID: file 'subsystem_vendor', 0x prefix
        Subsystem Device ID: file 'subsystem_device', 0x prefix
        Device Class: file 'class', 0x prefix
        Revision ID: file 'revision', 0x prefix
    */

    for directory in read_dir("/sys/bus/pci/devices/").unwrap() {
        let label = String::from("");
        // TODO: Figure out what the FUCK a label is supposed to be/do, and how to obtain it.
        let vendor_id = get_pci_device_attribute!(u16, &directory, "vendor")?; // Vendor ID
        let device_id = get_pci_device_attribute!(u16, &directory, "device")?; // Device ID
        let subsys_device_id = get_pci_device_attribute!(u16, &directory, "subsystem_device")?; // Subsystem Device ID
        let subsys_vendor_id = get_pci_device_attribute!(u16, &directory, "subsystem_vendor")?; // Subsystem Vendor ID

        let class_code = get_pci_device_attribute!(u32, &directory, "class")?;
        let class: u8 = ((class_code >> 16) & 0xFF) as u8; // Device Class
        let subclass: u8 = ((class_code >> 8) & 0xFF) as u8; // Device Subclass
        let programming_interface: u8 = (class_code & 0xFF) as u8; // Device Programming Interface

        let revision_id = get_pci_device_attribute!(u8, &directory, "revision")?; // Revision ID

        let components =
            comps_from_linux_pci_addr(directory.unwrap().file_name().to_str().unwrap()).unwrap();
        // TODO: handle in case of error as to not panic on unwrap.
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
pub fn _get_pci_by_id(_vendor: u16, _device: u16) -> Result<PciDevice, PciEnumerationError> {
    todo!()
}
