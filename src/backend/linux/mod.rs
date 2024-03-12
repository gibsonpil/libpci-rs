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

use crate::backend::all_fields_available;
use crate::pci::*;
use std::fs::*;

// ahaha this particular code is by Shibe Drill

/// Internal macro to get an integer of a specific type from a file in a
/// directory. The integer type must implement from_str_radix().
/// The path must be specified as a valid Path, and the attribute should
/// be a string representing a filename.
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

/// Primary Linux backend functionality. Iterates through /sys/bus/pci/devices
/// and gets information from it. Each directory is an address, and each file
/// in that directory is information about the device. These files contain hex
/// strings prefixed by 0x. The file names are the names of the attributes,
/// and the numbers contained inside them are the values of those attributes.
#[inline]
pub fn _get_pci_list() -> Result<Vec<PciDeviceHardware>, PciEnumerationError> {
    let mut device_list: Vec<PciDeviceHardware> = Vec::new();

    for directory in read_dir("/sys/bus/pci/devices/")? {
        // Class contains multiple items: class, subclass, and the programming
        // interface.
        let class_code = get_pci_device_attribute!(u32, &directory, "class")?;

        device_list.push(PciDeviceHardware {
            address: PciDeviceAddress::try_from(
                // try_from parses an address in the 0000:00:00.0 format
                directory
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .into_string()
                    .unwrap(),
            )
            .ok(), // TODO: delete this when we change from an
            // Option<PciDeviceAddress> to a Result<PciDeviceAddress,
            // PciInformationError>
            vendor_id: get_pci_device_attribute!(u16, &directory, "vendor")?, // Vendor ID
            device_id: get_pci_device_attribute!(u16, &directory, "device")?, // Device ID
            subsys_device_id: get_pci_device_attribute!(u16, &directory, "subsystem_device")?, // Subsystem Device ID
            subsys_vendor_id: get_pci_device_attribute!(u16, &directory, "subsystem_vendor")?, // Subsystem Vendor ID
            class: ((class_code >> 16) & 0xFF) as u8, // Class
            subclass: ((class_code >> 8) & 0xFF) as u8, // Subclass
            programming_interface: (class_code & 0xFF) as u8, // Programming Interface
            revision_id: get_pci_device_attribute!(u8, &directory, "revision")?, // Revision ID
        })
    }

    // return the list at the end once all the devices are in it.
    Ok(device_list)
}

pub fn _get_field_availability() -> PciDeviceHardware {
    all_fields_available()
}
