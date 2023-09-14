use std::{num::ParseIntError, fs::{DirEntry, read_to_string}};

// Define a PCI device as its component fields
#[derive(Debug)]
pub struct PciDevice {
    pub slot: String,
    pub label: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub subsys_device_id: u16,
    pub subsys_vendor_id: u16,
    pub device_class: u32,
    pub revision_id: u8,
}

// Two helper functions to convert hexadecimal IDs into numbers.
// I predict these will be helpful for all platforms, not just Linux.
pub fn ox_hex_string_to_u8(input_string: &str) -> Result<u8, ParseIntError> {
    let input_string = if input_string.starts_with("0x") {
        &input_string[2..]
    } else {
        input_string
    }.trim();
    u8::from_str_radix(input_string, 8)
}
pub fn ox_hex_string_to_u16(input_string: &str) -> Result<u16, ParseIntError> {
    let input_string = if input_string.starts_with("0x") {
        &input_string[2..]
    } else {
        input_string
    }.trim();
    u16::from_str_radix(input_string, 16)
}

pub fn ox_hex_string_to_u32(input_string: &str) -> Result<u32, ParseIntError> {
    let input_string = if input_string.starts_with("0x") {
        &input_string[2..]
    } else {
        input_string
    }.trim();
    u32::from_str_radix(input_string, 32)
}


pub enum GetPciDevAttrErr {
    ReadDirError,
    ReadFileError,
    ParseHexError,
}

pub fn get_pci_device_attribute_u8(dir: &Result<DirEntry, std::io::Error>, attribute: &str) -> Result<u8, GetPciDevAttrErr> {
    if let Ok(dir_usable) = dir {
        if let Ok(file_contents) = read_to_string(format!("{}/{}", dir_usable.path().to_string_lossy(), attribute)) {
            if let Ok(decoded_number) = ox_hex_string_to_u8(&file_contents) {
                return Ok(decoded_number)
            } else {
                return Err(GetPciDevAttrErr::ParseHexError)
            }
        } else {
            return Err(GetPciDevAttrErr::ReadFileError)
        }
    } else {
        return Err(GetPciDevAttrErr::ReadDirError)
    }
}

pub fn get_pci_device_attribute_u16(dir: &Result<DirEntry, std::io::Error>, attribute: &str) -> Result<u16, GetPciDevAttrErr> {
    if let Ok(dir_usable) = dir {
        if let Ok(file_contents) = read_to_string(format!("{}/{}", dir_usable.path().to_string_lossy(), attribute)) {
            if let Ok(decoded_number) = ox_hex_string_to_u16(&file_contents) {
                return Ok(decoded_number)
            } else {
                return Err(GetPciDevAttrErr::ParseHexError)
            }
        } else {
            return Err(GetPciDevAttrErr::ReadFileError)
        }
    } else {
        return Err(GetPciDevAttrErr::ReadDirError)
    }
}

pub fn get_pci_device_attribute_u32(dir: &Result<DirEntry, std::io::Error>, attribute: &str) -> Result<u32, GetPciDevAttrErr> {
    if let Ok(dir_usable) = dir {
        if let Ok(file_contents) = read_to_string(format!("{}/{}", dir_usable.path().to_string_lossy(), attribute)) {
            if let Ok(decoded_number) = ox_hex_string_to_u32(&file_contents) {
                return Ok(decoded_number)
            } else {
                return Err(GetPciDevAttrErr::ParseHexError)
            }
        } else {
            return Err(GetPciDevAttrErr::ReadFileError)
        }
    } else {
        return Err(GetPciDevAttrErr::ReadDirError)
    }
}