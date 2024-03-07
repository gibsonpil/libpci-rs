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

use crate::class::get_class;
use core::fmt;
use std::fmt::Display;
use std::io::ErrorKind;
use std::num::ParseIntError;

/// A struct representing a PCI device, all its hardcoded information, and its
/// location on the system's PCI device bus.
#[derive(Debug, Clone)]
pub struct PciDeviceHardware {
    /// One of a set of "segments" containing multiple PCI buses.
    pub domain: u32,
    /// A specific bus to handle PCI device connections.
    pub bus: u8,
    /// A specific device on a PCI bus.
    pub device: u8,
    /// An even more specific sub-function of a PCI device. Graphics cards often have 2, for graphics and sound.
    pub function: u8,
    /// I'm not even sure what this is.
    pub label: String,
    /// The ID of the device manufacturer.
    pub vendor_id: u16,
    /// The ID of the device.
    pub device_id: u16,
    /// The ID of the sub-device.
    pub subsys_device_id: u16,
    /// The ID of the sub-device vendor (normally the same as the device vendor).
    pub subsys_vendor_id: u16,
    /// A category of functionality that the device provides.
    pub class: u8,
    /// A more specific category of functionality, organized by class.
    pub subclass: u8,
    /// An even more specific subcategory of functionality, defining how the device is programmed.
    pub programming_interface: u8,
    /// The device's hardware revision.
    pub revision_id: u8,
}

impl Display for PciDeviceHardware {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04x}:{:02x}:{:02x}.{:x} {}: \n\tVID={:04x} DID={:04x} SVID={:04x} SDID={:04x} Class={:02x} Subclass={:02x} PIF={:02x} Rev={:02x}",  self.domain, self.bus, self.device, self.function, get_class(self.class).unwrap().get_name(), self.vendor_id, self.device_id, self.subsys_vendor_id, self.subsys_device_id, self.class as u32, self.subclass, self.programming_interface, self.revision_id)
    }
}

/// Get all the installed PCI devices in the system.
///
/// Returns a `PciEnumerationError` or a `Vec<PciDeviceHardware`, containing
/// representations of every PCI device installed in the system.
pub use crate::backend::get_pci_list;

/// A list of errors that can occur while enumerating PCI devices.
#[derive(Debug)]
pub enum PciEnumerationError {
    /// Error interfacing with OS APIs.
    OsError,
    /// Some kind of IO error.
    GenericIoError(std::io::Error),
    /// Unable to read a directory.
    ReadDirectory,
    /// PCI device, attribute, directory, or file missing
    NotFound,
    /// No permission to perform operation. Mostly for use on non-Linux POSIX systems.
    PermissionDenied,
    /// Attribute is not valid hex.
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_pci_listing() {
        println!("Begin test output: test_pci_listing");
        let device_list = crate::backend::get_pci_list().unwrap();
        for device in device_list {
            println!("{}", device);
        }
        println!("End test output: test_pci_listing");
    }
}
