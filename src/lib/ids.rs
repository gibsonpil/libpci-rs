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

//! # About this module
//! The functions, structures, and methods here can be used to look up info
//! about a PCI device that might be physically installed in a system, or
//! might not be (and you just have its ID numbers). Let's say you want info
//! on a device you don't have installed, with VID `0x8086` and DID `0xA0F0`,
//! you can chain these calls to get a device or vendor entry like so:
//! ```rust
//! # use libpci_rs::ids::{PciDeviceEntry, PciVendorEntry, lookup_vendor};
//! let ven_entry: PciVendorEntry = lookup_vendor(0x8086).unwrap();
//! let dev_entry: &PciDeviceEntry = ven_entry.device(0xA0F0).unwrap();
//! assert_eq!("Wi-Fi 6 AX201", dev_entry.name());
//! ```
//! Similarly, you can get an ID entry regarding a vendor, and get its name:
//! ```rust
//! # use libpci_rs::ids::{PciVendorEntry, lookup_vendor};
//! let ven_entry: PciVendorEntry = lookup_vendor(0x8086).unwrap();
//! assert_eq!("Intel Corporation", ven_entry.name());
//! ```
//! This code is also used behind-the-scenes in the methods of
//! [PciDeviceHardware](crate::pci::PciDeviceHardware), so you can easily
//! obtain info about a device that is physically present. You don't even
//! need to know its IDs to get the info.
//! ```rust
//! # use libpci_rs::pci::{PciDeviceHardware, get_pci_list};
//! let pci_list: Vec<PciDeviceHardware> = get_pci_list().unwrap();
//! let pci_device: &PciDeviceHardware = pci_list.get(0).unwrap();
//! println!("{}", pci_device.device_name().unwrap_or("Unknown device name".to_string()));
//! ```

#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/pci_devices_phf.rs"));

/// An ID entry representing a PCI device vendor.
#[derive(Copy, Clone)]
pub struct PciVendorEntry {
    /// The integer vendor ID.
    id: u16,
    /// The name of the vendor.
    name: &'static str,
    /// The list of devices manufactured by the vendor.
    devices: &'static [PciDeviceEntry],
}

/// An ID entry representing a PCI device.
#[derive(Copy, Clone)]
pub struct PciDeviceEntry {
    /// The integer device ID.
    id: u16,
    /// The name of the device.
    name: &'static str,
    /// The list of possible subsystems for the device.
    subsystems: &'static [PciSubsystemEntry],
}

/// An ID entry representing a PCI device subsystem.
#[derive(Copy, Clone)]
pub struct PciSubsystemEntry {
    /// The integer subvendor ID.
    subvendor: u16,
    /// The integer subdevice ID.
    subdevice: u16,
    /// The subsystem name.
    name: &'static str,
}

/// Gets a vendor with a given ID, if there is one.
pub fn lookup_vendor(vid: u16) -> Option<PciVendorEntry> {
    let result = VENDORS.get(&vid);
    Some(*result?)
}

impl PciVendorEntry {
    /// Returns the vendor ID.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns the vendor name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Gets a specific device by ID.
    pub fn device(&self, did: u16) -> Option<&PciDeviceEntry> {
        self.devices.iter().find(|x| x.id == did)
    }

    /// Gets all devices associated with a vendor.
    pub fn devices(&self) -> Option<Vec<&PciDeviceEntry>> {
        let ret: Vec<&PciDeviceEntry> = self.devices.iter().collect();
        match ret.is_empty() {
            true => None,
            false => Some(ret),
        }
    }
}

impl PciDeviceEntry {
    /// Returns the device ID.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns the device name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Gets all the subsystems associated with a device. Many devices do not
    /// have subsystems, so it is common for this function to return None.
    pub fn subsystems(&self) -> Option<Vec<&PciSubsystemEntry>> {
        let ret: Vec<&PciSubsystemEntry> = self.subsystems.iter().collect();
        match ret.is_empty() {
            true => None,
            false => Some(ret),
        }
    }

    /// Gets a specific subsystem by ID. Many devices do not have subsystems,
    /// so it is common for this function to return None.
    pub fn subsystem(&self, did: u16, vid: u16) -> Option<&PciSubsystemEntry> {
        self.subsystems
            .iter()
            .find(|x| x.subdevice == did && x.subvendor == vid)
    }
}

impl PciSubsystemEntry {
    /// Returns the subsystem vendor.
    pub fn subvendor(&self) -> u16 {
        self.subvendor
    }

    /// Returns the subsystem device.
    pub fn subdevice(&self) -> u16 {
        self.subdevice
    }

    /// Returns the subsystem name.
    pub fn name(&self) -> &'static str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::ids::lookup_vendor;

    /// Test looking up a specific vendor.
    #[test]
    fn test_lookup_vendor() {
        let vendor = lookup_vendor(20).unwrap();
        assert_eq!(vendor.name(), "Loongson Technology LLC");
    }

    /// Test looking up a specific device.
    #[test]
    fn test_get_device() {
        let vendor = lookup_vendor(0x10de).unwrap();
        let device = vendor.device(0x1056).unwrap();
        assert_eq!(device.name(), "GF119M [NVS 4200M]");
    }

    /// Test looking up all the information of every device in the system.
    #[test]
    fn test_pci_listing_pretty() {
        println!("Begin test output: test_pci_listing_pretty");
        let device_list = crate::pci::get_pci_list().unwrap();
        for device in device_list {
            println!(
                "{}",
                device.pretty_print().unwrap_or_else(|| format!(
                    "Could not obtain pretty-print for device ({:04x}:{:04x}).",
                    device.vendor_id, device.device_id
                ))
            );
        }
        println!("End test output: test_pci_listing_pretty");
    }

    /// Test looking up all the subdevice information of every device in the
    /// system.
    #[test]
    fn test_lookup_subdevice() {
        println!("Begin test output: test_lookup_subdevice");
        let device_list = crate::pci::get_pci_list().unwrap();
        for device in device_list {
            println!(
                "{} {} {}",
                device
                    .vendor_name()
                    .unwrap_or("(no vendor name)".to_string()),
                device
                    .subclass_name()
                    .unwrap_or("(no subclass name)".to_string()),
                device
                    .subdevice_name()
                    .unwrap_or("(no subdevice name)".to_string())
            );
        }
        println!("End test output: test_lookup_subdevice");
    }
}
