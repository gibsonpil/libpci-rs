// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

#![allow(dead_code)]

pub use crate::pci::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android"))] {
        mod linux;
        use linux::{_get_field_availability, _get_pci_list};
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        use crate::backend::windows::{_get_field_availability, _get_pci_list};
    } else {
        // It is safe to assume we are probably running under a C++ backend.
        mod bridge;
        use bridge::{_get_pci_list, _get_field_availability};
    }
}

pub fn get_pci_list() -> Result<Vec<PciDeviceHardware>, PciEnumerationError> {
    _get_pci_list()
}

/// Returns a PciDeviceHardware object in which available fields are set to
/// zero and unavailable field are set to a value in the PciInformationError enum.
pub fn get_field_availability() -> PciDeviceHardware {
    _get_field_availability()
}

/// Code for use within backend modules. Simply sets returns a
/// PciDeviceHardware object with all fields set to zero.
pub(crate) fn all_fields_available() -> PciDeviceHardware {
    // Fields are zero-initialized by default.
    PciDeviceHardware::default()
}
