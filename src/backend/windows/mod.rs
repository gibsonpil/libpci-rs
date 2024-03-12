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

use std::collections::HashMap;
use std::mem::size_of;

use windows::core::HSTRING;
use windows::Win32::Devices::DeviceAndDriverInstallation::{
    SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW,
    SetupDiGetDeviceRegistryPropertyA, DIGCF_ALLCLASSES, DIGCF_PRESENT, SPDRP_ADDRESS,
    SPDRP_BUSNUMBER, SPDRP_HARDWAREID, SP_DEVINFO_DATA,
};

use crate::backend::all_fields_available;
use crate::pci::*;

impl From<windows::core::Error> for PciEnumerationError {
    fn from(_err: windows::core::Error) -> Self {
        // TODO: log more detailed error.
        PciEnumerationError::OsError
    }
}

#[inline]
pub fn _get_pci_list() -> Result<Vec<PciDeviceHardware>, PciEnumerationError> {
    let mut result: Vec<PciDeviceHardware> = Vec::new();

    unsafe {
        let device_info = SetupDiGetClassDevsW(
            None,
            &HSTRING::from("PCI"),
            None,
            DIGCF_ALLCLASSES | DIGCF_PRESENT,
        )?;

        let mut device_info_data: SP_DEVINFO_DATA = SP_DEVINFO_DATA {
            cbSize: size_of::<SP_DEVINFO_DATA>() as u32,
            ClassGuid: Default::default(),
            DevInst: 0,
            Reserved: 0,
        };

        let mut win_bus: u32 = 0;
        let mut win_addr: u32 = 0;

        let mut i = 0;
        while SetupDiEnumDeviceInfo(device_info, i, &mut device_info_data).is_ok() {
            SetupDiGetDeviceRegistryPropertyA(
                device_info,
                &device_info_data,
                SPDRP_BUSNUMBER,
                None,
                Some(std::mem::transmute::<&mut u32, &mut [u8; 4]>(&mut win_bus)),
                None,
            )?;

            SetupDiGetDeviceRegistryPropertyA(
                device_info,
                &device_info_data,
                SPDRP_ADDRESS,
                None,
                Some(std::mem::transmute::<&mut u32, &mut [u8; 4]>(&mut win_addr)),
                None,
            )?;

            // Request size of SPDRP_HARDWAREID from Windows.
            let mut win_hwid_size: u32 = 0;
            let _ = SetupDiGetDeviceRegistryPropertyA(
                device_info,
                &device_info_data,
                SPDRP_HARDWAREID,
                None,
                None,
                Some(&mut win_hwid_size),
            );

            // Allocate a buffer on the heap and get the value.
            let mut win_hwid: Box<[u8]> = vec![0; win_hwid_size as usize].into_boxed_slice();
            SetupDiGetDeviceRegistryPropertyA(
                device_info,
                &device_info_data,
                SPDRP_HARDWAREID,
                None,
                Some(&mut win_hwid),
                None,
            )?;

            /*
            The data we want comes in the form of a set of strings.
            They look like this:
                VEN_10EC&DEV_5765&SUBSYS_576510EC&REV_01
                VEN_10EC&DEV_5765&CC_010802
                ...
            The first string contains a lot of the info we need.
            VEN: vendor id
            DEV: device id
            SUBSYS: highest 16 bits are the subsystem device id, lowest 16 bits are the subsystem vendor ID
            REV: revision
            The SECOND or THIRD string, though, contains the device class instead of the subsystem.
            CC: Holds the device class, subclass, and POSSIBLY programming interface.
            I don't know why the data comes like this, in the form of a utf-8 encoded string chock full
            of null characters, but what do we expect of Microsoft?
            */

            // String conversion, trim end, splitting at null terminator and removing the PCI\ prefix.
            let win_hwid_entries: Vec<&str> = std::str::from_utf8(&win_hwid)
                .unwrap()
                .split('\0')
                .map(|s| s.strip_prefix("PCI\\").unwrap_or(""))
                .filter(|s| !s.is_empty()) // Filter out empty strings.
                .collect();

            // Declare this map and then push to it with every single item in the HWID's entries.
            // That way we get all the usable and unique data we could need.
            let mut values_mapping: HashMap<&str, u32> = HashMap::new();

            for device in win_hwid_entries {
                for kv in device.split('&') {
                    let delimiter = kv.find('_').unwrap();
                    let key = &kv[0..delimiter];
                    let value = u32::from_str_radix(&kv[delimiter + 1..kv.len()], 16).unwrap();
                    values_mapping.insert(key, value);
                }
            }

            // Have to perform some bitwise ops on these two so we make them their own variables
            let subsys = values_mapping.get("SUBSYS").unwrap();
            let cc = values_mapping.get("CC").unwrap();

            result.push(PciDeviceHardware {
                address: Some(PciDeviceAddress {
                    domain: (win_bus >> 8) & 0xFFFFFF, // Domain is in high 24 bits of SPDRP_BUSNUMBER.
                    bus: (win_bus & 0xFF) as u8,       // Bus is in low 8 bits of SPDRP_BUSNUMBER.
                    device: ((win_addr >> 16) & 0xFF) as u8, // Device (u8) is in high 16 bits of SPDRP_ADDRESS.
                    function: (win_addr & 0xFF) as u8, // Function (u8) is in low 16 bits of SDRP_ADDRESS.
                }),
                vendor_id: *values_mapping
                    .get("VEN")
                    .ok_or(PciEnumerationError::NotFound)? as u16,
                device_id: *values_mapping
                    .get("DEV")
                    .ok_or(PciEnumerationError::NotFound)? as u16,
                subsys_device_id: (subsys >> 16) as u16, // High 16 bits of SUBSYS.
                subsys_vendor_id: (subsys & 0xFFFF) as u16, // Low 16 bits of SUBSYS.
                class: ((cc & 0x00FF00) >> 8) as u8,     // Middle 8 bits of CC.
                subclass: (cc & 0x0000FF) as u8,         // Last 8 bits of CC.
                programming_interface: ((cc & 0xFF0000) >> 16) as u8, // High 8 bits of CC????? Unsure!
                revision_id: *values_mapping
                    .get("REV")
                    .ok_or(PciEnumerationError::NotFound)? as u8,
            });

            i += 1;
        }

        SetupDiDestroyDeviceInfoList(device_info)?;
    };

    Ok(result)
}

pub fn _get_field_availability() -> PciDeviceHardware {
    all_fields_available()
}
