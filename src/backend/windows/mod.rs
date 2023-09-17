// Copyright (c) 2023 NamedNeon. All rights reserved.
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

use std::mem::size_of;

use windows::Win32::Devices::DeviceAndDriverInstallation::{DIGCF_ALLCLASSES, DIGCF_PRESENT, SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW, SetupDiGetDeviceRegistryPropertyW, SP_DEVINFO_DATA, SPDRP_ADDRESS, SPDRP_BUSNUMBER};
use windows::core::HSTRING;

use crate::backend::common::{PciDevice, PciEnumerationError};

impl From<windows::core::Error> for PciEnumerationError {
    fn from(_err: windows::core::Error) -> Self {
        // TODO: log more detailed error.
        PciEnumerationError::OsError
    }
}

#[inline]
pub fn _get_pci_list() -> Result<Vec<PciDevice>, PciEnumerationError> {
    let mut result: Vec<PciDevice> = Vec::new();

    unsafe {
        let device_info = SetupDiGetClassDevsW(None, &HSTRING::from("PCI"), None, DIGCF_ALLCLASSES | DIGCF_PRESENT)?;

        let mut device_info_data: SP_DEVINFO_DATA = SP_DEVINFO_DATA {
            cbSize: size_of::<SP_DEVINFO_DATA>() as u32,
            ClassGuid: Default::default(),
            DevInst: 0,
            Reserved: 0,
        };

        let mut win_bus: u32 = 0;
        let mut win_addr: u32 = 0;
        let mut win_hwid: [u8; 512] = [0; 512]; // TODO: parse.

        for i in 0.. {
            if SetupDiEnumDeviceInfo(device_info, i, &mut device_info_data).is_err() {
                // We either don't have any items left or ran into a problem.
                break;
            }

            SetupDiGetDeviceRegistryPropertyW(
                device_info,
                &device_info_data,
                SPDRP_BUSNUMBER,
                None,
                Some(std::mem::transmute::<&mut u32, &mut [u8; 4]>(&mut win_bus)),
                None
            )?;

            SetupDiGetDeviceRegistryPropertyW(
                device_info,
                &device_info_data,
                SPDRP_ADDRESS,
                None,
                Some(std::mem::transmute::<&mut u32, &mut [u8; 4]>(&mut win_addr)),
                None
            )?;

            SetupDiGetDeviceRegistryPropertyW(
                device_info,
                &device_info_data,
                SPDRP_ADDRESS,
                None,
                Some(&mut win_hwid),
                None
            )?;

            result.push(
                PciDevice {
                    domain: (win_bus >> 8) & 0xFFFFFF, // Domain is in high 24 bits of SPDRP_BUSNUMBER.
                    bus: (win_bus & 0xFF) as u8, // Bus is in low 8 bits of SPDRP_BUSNUMBER.
                    device: ((win_addr >> 16) &0xFF) as u8, // Device (u8) is in high 16 bits of SPDRP_ADDRESS.
                    function: (win_addr & 0xFF) as u8, // Function (u8) is in low 16 bits of SDRP_ADDRESS.
                    label: "".to_string(),
                    vendor_id: 0,
                    device_id: 0,
                    subsys_device_id: 0,
                    subsys_vendor_id: 0,
                    class: 0,
                    subclass: 0,
                    programming_interface: 0,
                    revision_id: 0,
                }
            );
        }

        SetupDiDestroyDeviceInfoList(device_info)?;
    };

    Ok(result)
}

#[inline]
pub fn _get_pci_by_id(vendor: u16, device: u16) -> Result<PciDevice, PciEnumerationError> {
    todo!()
}


