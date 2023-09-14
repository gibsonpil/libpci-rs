use std::{num::ParseIntError, fs::{DirEntry, read_to_string}};
use std::io::ErrorKind;

#[derive(Debug)]
pub enum PciEnumerationError {
    OsError,
    GenericIoError(std::io::Error),
    ReadDirectory,
    NotFound,
    PermissionDenied,
    ParseInt(ParseIntError),
}

// Convert IO errors to PCI enumeration errors.
impl From<std::io::Error> for PciEnumerationError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            ErrorKind::NotFound => PciEnumerationError::NotFound,
            ErrorKind::PermissionDenied => PciEnumerationError::PermissionDenied,
            _ => PciEnumerationError::GenericIoError(err),
        }
    }
}

// Convert integer parsing error into PCI enumeration error.
impl From<ParseIntError> for PciEnumerationError {
    fn from(err: ParseIntError) -> Self {
        PciEnumerationError::ParseInt(err)
    }
}

// Define a PCI device as its component fields
#[derive(Debug, Clone)]
pub struct PciDevice {
    pub domain: u32,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub label: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub subsys_device_id: u16,
    pub subsys_vendor_id: u16,
    pub device_class: u32,
    pub revision_id: u8,
}

pub(crate) fn ox_hex_string_to_u8(input_string: &str) -> Result<u8, ParseIntError> {
    let input_string = if input_string.starts_with("0x") {
        &input_string[2..]
    } else {
        input_string
    }.trim();
    u8::from_str_radix(input_string, 16)
}

pub(crate) fn ox_hex_string_to_u16(input_string: &str) -> Result<u16, ParseIntError> {
    let input_string = if input_string.starts_with("0x") {
        &input_string[2..]
    } else {
        input_string
    }.trim();
    u16::from_str_radix(input_string, 16)
}

pub(crate) fn ox_hex_string_to_u32(input_string: &str) -> Result<u32, ParseIntError> {
    let input_string = if input_string.starts_with("0x") {
        &input_string[2..]
    } else {
        input_string
    }.trim();
    u32::from_str_radix(input_string, 16)
}

pub(crate) fn get_pci_device_attribute_u8(dir: &Result<DirEntry, std::io::Error>, attribute: &str) -> Result<u8, PciEnumerationError> {
    let dir_usable = match dir {
        Ok(f) => f,
        Err(_) => {
            return Err(PciEnumerationError::ReadDirectory);
        }
    };

    let file_contents = read_to_string(format!("{}/{}", dir_usable.path().to_string_lossy(), attribute))?;
    let decoded_number = ox_hex_string_to_u8(&file_contents)?;

    Ok(decoded_number)
}

pub(crate) fn get_pci_device_attribute_u16(dir: &Result<DirEntry, std::io::Error>, attribute: &str) -> Result<u16, PciEnumerationError> {
    let dir_usable = match dir {
        Ok(f) => f,
        Err(_) => {
            return Err(PciEnumerationError::ReadDirectory);
        }
    };

    let file_contents = read_to_string(format!("{}/{}", dir_usable.path().to_string_lossy(), attribute))?;
    let decoded_number = ox_hex_string_to_u16(&file_contents)?;

    Ok(decoded_number)
}

pub(crate) fn get_pci_device_attribute_u32(dir: &Result<DirEntry, std::io::Error>, attribute: &str) -> Result<u32, PciEnumerationError> {
    let dir_usable = match dir {
        Ok(f) => f,
        Err(_) => {
            return Err(PciEnumerationError::ReadDirectory);
        }
    };

    let file_contents = read_to_string(format!("{}/{}", dir_usable.path().to_string_lossy(), attribute))?;
    let decoded_number = ox_hex_string_to_u32(&file_contents)?;

    Ok(decoded_number)
}

#[cfg(test)]
mod tests {
    use crate::backend::common::{ox_hex_string_to_u16, ox_hex_string_to_u32, ox_hex_string_to_u8};

    #[test]
    fn test_hex_decoding() {
        // Test to make sure every bit is recognized using the highest possible integer!
        assert_eq!(ox_hex_string_to_u8("0xFF"), Ok(255));
        assert_eq!(ox_hex_string_to_u16("0xFFFF"), Ok(65535));
        assert_eq!(ox_hex_string_to_u32("0xFFFFFFFF"), Ok(4294967295));
    }
}