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

use crate::backend::PciEnumerationError;
use crate::pci::PciDeviceHardware;

#[cxx::bridge]
mod ffi {
    struct CXXPciDeviceHardware {
        domain: u32,
        bus: u8,
        device: u8,
        function: u8,
        vendor_id: u16,
        device_id: u16,
        subsys_device_id: u16,
        subsys_vendor_id: u16,
        class_id: u8,
        subclass: u8,
        programming_interface: u8,
        revision_id: u8,
    }

    unsafe extern "C++" {
        include!("libpci-rs/src/backend/include/common.h");

        fn _get_pci_list() -> Vec<CXXPciDeviceHardware>;
    }
}

pub fn _get_pci_list() -> Result<Vec<PciDeviceHardware>, PciEnumerationError> {
    let mut result: Vec<PciDeviceHardware> = vec![];
    let list = ffi::_get_pci_list();

    for device in list {
        result.push(PciDeviceHardware {
            domain: device.domain,
            bus: device.bus,
            device: device.device,
            function: device.function,
            vendor_id: device.vendor_id,
            device_id: device.device_id,
            subsys_device_id: device.subsys_device_id,
            subsys_vendor_id: device.subsys_vendor_id,
            class: device.class_id,
            subclass: device.subclass,
            programming_interface: device.programming_interface,
            revision_id: device.revision_id,
        });
    }

    Ok(result)
}

