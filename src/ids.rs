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

pub fn get_device(vendor_id: u16, device_id: u16) -> PciDeviceEntry {
    let vendor_entry = VENDORS.get(&vendor_id);
    *vendor_entry
        .unwrap()
        .devices
        .iter()
        .find(|dev| dev.id == device_id)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::ids::get_device;

    #[test]
    fn test_get_device() { 
        let dev = get_device(4318, 7810);
        assert_eq!(dev.name, "TU104 [GeForce RTX 2080]");
    }
}

