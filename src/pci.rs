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

use core::fmt;
use std::fmt::Display;
use crate::class::get_class;

#[derive(Debug, Clone)]
pub struct PciDevice {
    
}

#[derive(Debug, Clone)]
pub struct PciDeviceHardware {
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

impl Display for PciDeviceHardware {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04x}:{:02x}:{:02x}.{:x} {}: \n\tVID={:04x} DID={:04x} SVID={:04x} SDID={:04x} Class={:02x} Subclass={:02x} PIF={:02x} Rev={:02x}",  self.domain, self.bus, self.device, self.function, get_class(self.class).unwrap().get_name(), self.vendor_id, self.device_id, self.subsys_vendor_id, self.subsys_device_id, self.class as u32, self.subclass, self.programming_interface, self.revision_id)
    }
}
