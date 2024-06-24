// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

//! # About this module
//! This module is the core of `libpci-rs`. It contains the most important
//! structs, methods, and functions that you might need to enumerate the PCI
//! devices on a system, or programmatically represent a PCI device in an
//! efficient data structure.

#[cfg(feature = "pciids")]
use crate::{class::*, ids::*};

use std::fmt::{Display, Formatter, Result};
use std::io::ErrorKind;
use std::num::ParseIntError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A structure representing the physical address of a PCI device.
pub struct PciDeviceAddress {
    /// One of a set of "segments" containing multiple PCI buses.
    pub domain: u32,
    /// A specific bus to handle PCI device connections.
    pub bus: u8,
    /// A specific device on a PCI bus.
    pub device: u8,
    /// An even more specific sub-function of a PCI device. Graphics cards
    /// often have 2, for graphics and sound.
    pub function: u8,
}

impl Display for PciDeviceAddress {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{:04x}:{:02x}:{:02x}.{:01x}",
            self.domain, self.bus, self.device, self.function,
        )
    }
}

impl TryFrom<String> for PciDeviceAddress {
    type Error = crate::pci::PciEnumerationError;
    fn try_from(
        address_string: String,
    ) -> std::result::Result<crate::pci::PciDeviceAddress, crate::pci::PciEnumerationError> {
        let parts: Vec<&str> = address_string.split(|c| c == ':' || c == '.').collect();

        // We know that if we somehow don't have all 4 segments of the address
        // then something has gone horribly wrong.
        if parts.len() != 4 {
            return Err(PciEnumerationError::NotFound);
        }

        Ok(PciDeviceAddress {
            domain: u32::from_str_radix(parts[0], 16)?,
            bus: u8::from_str_radix(parts[1], 16)?,
            device: u8::from_str_radix(parts[2], 16)?,
            function: u8::from_str_radix(parts[3], 16)?,
        })
    }
}

/// A struct representing a PCI device, all its hardcoded information, and its
/// location on the system's PCI device bus. It implements several methods to
/// get ID related information, gated behind the [pciids feature](crate#pciids).
///
/// # Field Availability
/// Some fields are not available on some platforms. Reference the below chart
/// to see which fields are available, unavailable without administrative
/// permission, or unavailable entirely. Each column represents all
/// architectures, except for those listed under the same OS in a different
/// column.
///
/// - Always: Available all the time without any elevated privileges.
/// - Elevated: Requires root / administrative permissions at runtime.
/// - Never: Not accessible on the platform.
///
///
/// | Field                 | Windows | Linux  | macOS  | macOS (ARM) | OpenBSD  | NetBSD   | DragonflyBSD | FreeBSD | Android  |
/// |-----------------------|---------|--------|--------|-------------|----------|----------|--------------|---------|----------|
/// | Address               | Always  | Always | Always | Never       | Elevated | Elevated | Always       | Always  | Elevated |
/// | Vendor ID             | Always  | Always | Always | Always      | Always   | Always   | Always       | Always  | Elevated |
/// | Device ID             | Always  | Always | Always | Always      | Always   | Always   | Always       | Always  | Elevated |
/// | Subvendor ID          | Always  | Always | Always | Always      | Always   | Always   | Always       | Always  | Elevated |
/// | Subdevice ID          | Always  | Always | Always | Always      | Always   | Always   | Always       | Always  | Elevated |
/// | Class                 | Always  | Always | Always | Always      | Always   | Always   | Always       | Always  | Elevated |
/// | Subclass              | Always  | Always | Always | Always      | Always   | Always   | Always       | Always  | Elevated |
/// | Programming Interface | Always  | Always | Always | Always      | Always   | Always   | Always       | Always  | Elevated |
/// | Revision              | Always  | Always | Always | Always      | Always   | Always   | Always       | Always  | Elevated |

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PciDeviceHardware {
    /// The address of a PCI device.  
    /// ***NOTICE:*** The [availability](#Availability) of this field varies by platform.
    // TODO: Make this a Result, propagate errors.
    pub address: Option<PciDeviceAddress>,
    /// The ID of the device manufacturer.
    pub vendor_id: u16,
    /// The ID of the device.
    pub device_id: u16,
    /// The ID of the sub-device.
    pub subsys_device_id: u16,
    /// The ID of the sub-device vendor.
    pub subsys_vendor_id: u16,
    /// A category of functionality that the device provides.
    pub class: u8,
    /// A more specific subcategory of the device class.
    pub subclass: u8,
    /// An even more specific subcategory of functionality, defining how the
    /// device is programmed.
    pub programming_interface: u8,
    /// The device's hardware revision.
    pub revision_id: u8,
}

impl Display for PciDeviceHardware {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: ({:04x}:{:04x}) SVID={:04x} SDID={:04x} Class={:02x} Subclass={:02x} PIF={:02x} Rev={:02x}",  
            if let Some(addr) = self.address {
                format!("{}", addr)
            } else {
                "[address inaccessible]".to_owned()
            },
            self.vendor_id,
            self.device_id,
            self.subsys_vendor_id,
            self.subsys_device_id,
            self.class,
            self.subclass,
            self.programming_interface,
            self.revision_id
        )
    }
}

/// All of the following methods in this block require the
/// [pciids feature](crate#pciids).
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
    /// Get a pretty representation of the entire device. This method does a
    /// lot of its own error handling, so if you want to handle things in
    /// a different way, you should just call the other  methods for the
    /// individual attributes. It will return [`None`] if any of the following
    /// items cannot be looked up:
    ///
    /// - The subclass name
    /// - The vendor name
    /// - The device name  
    ///
    /// The devices print in this style:  
    /// `f619:00:00.0 Communication controller: Red Hat, Inc. Virtio file system  (rev 01)`  
    /// Unless the address is not [available](crate::pci::PciDeviceHardware#availability):  
    /// `[address inaccessible] Bridge: Red Hat, Inc. Virtio 1.0 socket  (rev 01)`  
    pub fn pretty_print(&self) -> Option<String> {
        Some(format!(
            "{} {}: {} {} {}",
            if let Some(address) = self.address {
                format!("{}", address)
            } else {
                "[address inaccessible]".to_owned()
            },
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
/// Returns a [`PciEnumerationError`] or a [`Vec`]<[`PciDeviceHardware`]>,
/// containing representations of every PCI device installed in the system.
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
    /// An error that couldn't be resolved originating from a foreign backend.
    GenericForeignError,
}

impl Display for PciEnumerationError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::OsError => "OsError",
                Self::GenericIoError(_ioerr) => "GenericIoError",
                Self::ReadDirectory => "ReadDirectory",
                Self::NotFound => "NotFound",
                Self::PermissionDenied => "PermissionDenied",
                Self::ParseInt(_parserr) => "ParseIntError",
                Self::GenericForeignError => "GenericForeignError",
            }
        )
    }
}

impl std::error::Error for PciEnumerationError {}

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
            _ => Unknown,
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
