// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

use crate::backend::all_fields_available;
use crate::pci::*;
use std::fs::*;
use std::num::ParseIntError;

// ahaha this particular code is by Shibe Drill

// look ma! macros!
macro_rules! impl_trait_for_types {
    ($trait_name:ident for $($type:ty),*) => {
        $(
            impl $trait_name for $type {
                fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
                    Self::from_str_radix(src, radix)
                }
            }
        )*
    };
}

trait FromStrRadix {
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>
    where
        Self: Sized;
}

// WHY IS THIS NOT IN THE STANDARD LIBRARY
// I will cry if they don't give this to us.
impl_trait_for_types!(FromStrRadix for u8, u16, u32);

/// Internal function to get a PCI device attribute of a certain integer size,
/// from a file with a certain name, given the directory the file is in.
/// T must be in the list above where I had to MANUALLY IMPL MY OWN TRAIT so I
/// could genericize this function.
fn get_pci_device_attribute<T>(dir: &DirEntry, attribute: &str) -> Result<T, PciEnumerationError>
where
    T: FromStrRadix,
{
    let file_contents = read_to_string(format!("{}/{}", dir.path().to_string_lossy(), attribute))?;
    let input_string = if let Some(stripped) = file_contents.strip_prefix("0x") {
        stripped
    } else {
        &file_contents
    }
    .trim();
    Ok(T::from_str_radix(input_string, 16)?)
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
        let dir_unwrapped = directory?;
        // Class contains multiple items: class, subclass, and the programming
        // interface.
        let class_code: u32 = get_pci_device_attribute(&dir_unwrapped, "class")?;

        device_list.push(PciDeviceHardware {
            address: PciDeviceAddress::try_from(
                // try_from parses an address in the 0000:00:00.0 format
                dir_unwrapped.file_name().into_string().unwrap(), // How can we get rid of this
            )
            .ok(), // TODO: delete this when we change from an
            // Option<PciDeviceAddress> to a Result<PciDeviceAddress,
            // PciInformationError>
            vendor_id: get_pci_device_attribute(&dir_unwrapped, "vendor")?, // Vendor ID
            device_id: get_pci_device_attribute(&dir_unwrapped, "device")?, // Device ID
            subsys_device_id: get_pci_device_attribute(&dir_unwrapped, "subsystem_device")?, // Subsystem Device ID
            subsys_vendor_id: get_pci_device_attribute(&dir_unwrapped, "subsystem_vendor")?, // Subsystem Vendor ID
            class: ((class_code >> 16) & 0xFF) as u8, // Class
            subclass: ((class_code >> 8) & 0xFF) as u8, // Subclass
            programming_interface: (class_code & 0xFF) as u8, // Programming Interface
            revision_id: get_pci_device_attribute(&dir_unwrapped, "revision")?, // Revision ID
        })
    }

    // return the list at the end once all the devices are in it.
    Ok(device_list)
}

pub fn _get_field_availability() -> PciDeviceHardware {
    all_fields_available()
}
