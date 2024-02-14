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

use core::fmt;
use std::fmt::Display;
use std::io::ErrorKind;
use std::num::ParseIntError;

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
    pub class: u8,
    pub subclass: u8,
    pub programming_interface: u8,
    pub revision_id: u8,
}

#[macro_export]
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

impl Display for PciDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04x}:{:02x}:{:02x}.{:x} VID={:04x} DID={:04x} SVID={:04x} SDID={:02x} Class={:x} Subclass={:x} PIF={:x} Rev={:x}", self.domain, self.bus, self.device, self.function, self.vendor_id, self.device_id, self.subsys_vendor_id, self.subsys_device_id, self.class, self.subclass, self.programming_interface, self.revision_id)
    }
}
