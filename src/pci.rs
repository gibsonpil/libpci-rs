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

#[cfg(feature = "pciids")]
use crate::{class::*, ids::*};

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

impl Default for PciDeviceHardware {
    fn default() -> Self {
        PciDeviceHardware {
            domain: 0,
            bus: 0,
            device: 0,
            function: 0,
            vendor_id: 0,
            device_id: 0,
            subsys_device_id: 0,
            subsys_vendor_id: 0,
            class: 0,
            subclass: 0,
            programming_interface: 0,
            revision_id: 0
        }
    }
}

impl Display for PciDeviceHardware {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04x}:{:02x}:{:02x}.{:x}, class {}: \n\t({:04x}:{:04x}) SVID={:04x} SDID={:04x} Class={:02x} Subclass={:02x} PIF={:02x} Rev={:02x}",  self.domain, self.bus, self.device, self.function, self.class, self.vendor_id, self.device_id, self.subsys_vendor_id, self.subsys_device_id, self.class as u32, self.subclass, self.programming_interface, self.revision_id)
    }
}

#[cfg(feature = "pciids")]
impl PciDeviceHardware {
    /// Get the pretty name of the device.
    pub fn device_name(&self) -> Option<String> {
        Some(
            lookup_vendor(self.vendor_id)?
                .device(self.device_id)?
                .name()
                .to_owned(),
        )
    }
    /// Get the pretty name of the vendor.
    pub fn vendor_name(&self) -> Option<String> {
        Some(lookup_vendor(self.vendor_id)?.name().to_owned())
    }
    /// Get the description of the device class.
    pub fn class_name(&self) -> Option<String> {
        Some(lookup_class(self.class)?.name().to_owned())
    }
    /// Get the description of the device subclass.
    pub fn subclass_name(&self) -> Option<String> {
        Some(
            lookup_class(self.class)?
                .subclass(self.subclass)?
                .name()
                .to_owned(),
        )
    }
    /// Get the description of the device programming interface.
    pub fn progint_name(&self) -> Option<String> {
        Some(
            lookup_class(self.class)?
                .subclass(self.subclass)?
                .prog(self.programming_interface)?
                .name()
                .to_owned(),
        )
    }
    /// Get the pretty name of the subdevice.
    pub fn subdevice_name(&self) -> Option<String> {
        Some(
            lookup_vendor(self.vendor_id)?
                .device(self.device_id)?
                .subsystem(self.subsys_device_id, self.subsys_vendor_id)?
                .name()
                .to_owned(),
        )
    }
    /// Get a pretty representation of the entire device.
    pub fn pretty_print(&self) -> Option<String> {
        Some(format!(
            "{:04x}:{:02x}:{:02x}.{:x} {}: {} {} {}",
            self.domain,
            self.bus,
            self.device,
            self.function,
            self.subclass_name()?,
            self.vendor_name()?,
            self.device_name()?,
            {
                if self.revision_id != 0 {
                    format!(" (rev {:02x})", self.revision_id)
                } else {
                    "".to_string()
                }
            }
        ))
    }
}

/// Get all the installed PCI devices in the system.
///
/// Returns a `PciEnumerationError` or a `Vec<PciDeviceHardware`, containing
/// representations of every PCI device installed in the system.
pub use crate::backend::get_pci_list;
use crate::pci::PciInformationError::{PermissionDenied, Unavailable, Unknown};

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

// A list of errors that can occur when getting information from a PCI device.
#[derive(Debug)]
#[repr(u8)]
pub enum PciInformationError {
    /// libpci-rs was unable to fetch the information requested as the target OS doesn't make it available.
    Unavailable = 1,
    /// libpci-rs was unable to fetch the information requested due to a permission issue.
    PermissionDenied = 2,
    /// An unknown error occured.
    Unknown = 3,
}

// Convert raw u8 values to PciInformationError.
impl From<u8> for PciInformationError {
    fn from(value: u8) -> Self {
        match value {
            1 => Unavailable,
            2 => PermissionDenied,
            _ => Unknown
        }
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
