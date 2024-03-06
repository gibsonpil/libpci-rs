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

#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/pci_devices_phf.rs"));

#[derive(Copy, Clone)]
pub struct PciVendorEntry {
    id: u16,
    name: &'static str,
    devices: &'static [PciDeviceEntry]
}

#[derive(Copy, Clone)]
pub struct PciDeviceEntry {
    id: u16,
    name: &'static str,
    subsystems: &'static [PciSubsystemEntry]
}

#[derive(Copy, Clone)]
pub struct PciSubsystemEntry {
    subvendor: u16,
    subdevice: u16,
    name: &'static str,
}

pub fn get_vendor(id: u16) -> Option<PciVendorEntry> {
    let result = VENDORS.get(&id);
    result?;
    Some(*result.unwrap())
}

impl PciVendorEntry {
    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    /// Gets all the devices associated with a vendor.
    pub fn get_device(_id: u16) -> Option<PciDeviceEntry> {
        todo!()
    }
}

impl PciDeviceEntry {
    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    /// Gets all the subsystems associated with a device.
    pub fn get_subsystems(_id: u16) -> Option<PciSubsystemEntry> {
        todo!()
    }
}

impl PciSubsystemEntry {
    pub fn get_subvendor(&self) -> u16 {
        self.subvendor
    }
    
    pub fn get_subdevice(&self) -> u16 {
        self.subdevice
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::ids::get_vendor;

    #[test]
    fn test_get_device() { 
        let vendor = get_vendor(20).unwrap();
        assert_eq!(vendor.get_name(), "Loongson Technology LLC");
    }
}
