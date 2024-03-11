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

pub use crate::pci::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        use linux::{_get_field_availability, _get_pci_list};
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        use windows::{_get_field_availability, _get_pci_list};
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
