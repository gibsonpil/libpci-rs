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

use crate::backend;
use crate::backend::{PciDeviceAddress, PciEnumerationError};
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

    #[repr(u8)]
    enum CXXPciEnumerationError {
        Success,
        OsError,
        ReadDirectory,
        NotFound,
        PermissionDenied,
        GenericForeignError
    }

    extern "Rust" {
        fn all_fields_available() -> CXXPciDeviceHardware;
    }

    unsafe extern "C++" {
        include!("libpci-rs/src/backend/include/common.h");
        fn _get_pci_list(output: &mut Vec<CXXPciDeviceHardware>) -> CXXPciEnumerationError;
        fn _get_field_availability() -> CXXPciDeviceHardware;
    }
}

impl From<ffi::CXXPciDeviceHardware> for PciDeviceHardware {
    fn from(device: ffi::CXXPciDeviceHardware) -> Self {
        let mut address: Option<PciDeviceAddress> = None;

        if device.domain != 0 ||
            device.bus != 0 ||
            device.device != 0 ||
            device.function != 0 {
            address = Some(PciDeviceAddress{
                domain: device.domain,
                bus: device.bus,
                device: device.device,
                function: device.function
            })
        }

        PciDeviceHardware {
            address,
            vendor_id: device.vendor_id,
            device_id: device.device_id,
            subsys_device_id: device.subsys_device_id,
            subsys_vendor_id: device.subsys_vendor_id,
            class: device.class_id,
            subclass: device.subclass,
            programming_interface: device.programming_interface,
            revision_id: device.revision_id,
        }
    }
}

impl From<PciDeviceHardware> for ffi::CXXPciDeviceHardware {
    fn from(device: PciDeviceHardware) -> Self {
        let mut domain: u32 = 0;
        let mut bus: u8 = 0;
        let mut device_id: u8 = 0;
        let mut function: u8 = 0;

        // Move the device address over if it exists.
        if device.address.is_some() {
            let address = device.address.unwrap();
            domain = address.domain;
            bus = address.bus;
            device_id = address.device;
            function = address.function;
        }

        ffi::CXXPciDeviceHardware {
            domain,
            bus,
            device: device_id,
            function,
            vendor_id: device.vendor_id,
            device_id: device.device_id,
            subsys_device_id: device.subsys_device_id,
            subsys_vendor_id: device.subsys_vendor_id,
            class_id: device.class,
            subclass: device.subclass,
            programming_interface: device.programming_interface,
            revision_id: device.revision_id,
        }
    }
}

pub fn _get_pci_list() -> Result<Vec<PciDeviceHardware>, PciEnumerationError> {
    let mut result: Vec<PciDeviceHardware> = vec![];
    let mut output: Vec<ffi::CXXPciDeviceHardware> = vec![];
    let code = ffi::_get_pci_list(&mut output);

    for device in output {
        result.push(PciDeviceHardware::from(device));
    }

    Ok(result)
}

pub fn _get_field_availability() -> PciDeviceHardware {
    let availability = ffi::_get_field_availability();
    PciDeviceHardware::from(availability)
}

fn all_fields_available() -> ffi::CXXPciDeviceHardware {
    backend::all_fields_available().into()
}
