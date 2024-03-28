// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

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

    enum CXXPciEnumerationError {
        Success,
        OsError,
        ReadDirectory,
        NotFound,
        PermissionDenied,
        GenericForeignError,
    }

    extern "Rust" {
        fn all_fields_available() -> CXXPciDeviceHardware;
    }

    unsafe extern "C++" {
        include!("libpci-rs/src/lib/backend/include/common.h");
        fn _get_pci_list(output: &mut Vec<CXXPciDeviceHardware>) -> CXXPciEnumerationError;
        fn _get_field_availability() -> CXXPciDeviceHardware;
    }
}

impl From<ffi::CXXPciDeviceHardware> for PciDeviceHardware {
    fn from(device: ffi::CXXPciDeviceHardware) -> Self {
        let mut address: Option<PciDeviceAddress> = None;

        if device.domain != 0 || device.bus != 0 || device.device != 0 || device.function != 0 {
            address = Some(PciDeviceAddress {
                domain: device.domain,
                bus: device.bus,
                device: device.device,
                function: device.function,
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

impl From<ffi::CXXPciEnumerationError> for PciEnumerationError {
    fn from(value: ffi::CXXPciEnumerationError) -> Self {
        match value {
            ffi::CXXPciEnumerationError::OsError => PciEnumerationError::OsError,
            ffi::CXXPciEnumerationError::ReadDirectory => PciEnumerationError::ReadDirectory,
            ffi::CXXPciEnumerationError::NotFound => PciEnumerationError::NotFound,
            ffi::CXXPciEnumerationError::PermissionDenied => PciEnumerationError::PermissionDenied,
            ffi::CXXPciEnumerationError::GenericForeignError => {
                PciEnumerationError::GenericForeignError
            }
            _ => PciEnumerationError::GenericForeignError, // This shouldn't be reachable.
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

    // Handle errors.
    if code != ffi::CXXPciEnumerationError::Success {
        return Err(PciEnumerationError::from(code));
    }

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
