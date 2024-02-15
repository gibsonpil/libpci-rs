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

use std::collections::BTreeMap;
use std::mem::size_of;

use windows::core::HSTRING;
use windows::Win32::Devices::DeviceAndDriverInstallation::{
    SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW,
    SetupDiGetDeviceRegistryPropertyW, DIGCF_ALLCLASSES, DIGCF_PRESENT, SPDRP_ADDRESS,
    SPDRP_BUSNUMBER, SPDRP_HARDWAREID, SP_DEVINFO_DATA,
};

use utf16string::WStr;

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
        let mut win_hwid: [u8; 394] = [0; 394]; // TODO: parse.

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
                None,
            )?;

            SetupDiGetDeviceRegistryPropertyW(
                device_info,
                &device_info_data,
                SPDRP_ADDRESS,
                None,
                Some(std::mem::transmute::<&mut u32, &mut [u8; 4]>(&mut win_addr)),
                None,
            )?;

            SetupDiGetDeviceRegistryPropertyW(
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
            CC: First 8 bits are the device class, middle 8 bits are the subclass, and last 8 bits are the programming interface.
            I don't know why the data comes like this, in the form of a utf16-le encoded string chock full
            of null characters, but what do we expect of Microsoft?

            TODO: there is way too much garbage here that needs refactoring. Remove as many unwraps as possible,
            and return NotFound errors if an attribute is not found in the set.
            */
            // String conversion
            let unparsed_hwid: String = WStr::from_utf16le(&win_hwid)
                .unwrap()
                .to_utf8()
                .replace('\0', "");
            // Declare this map and then push to it with every single item in the HWID's entries.
            // That way we get all the usable and unique data we could need.
            let mut values_mapping: BTreeMap<&str, u32> = BTreeMap::new();
            for key_value_pair_set in unparsed_hwid.split("PCI\\") {
                if !key_value_pair_set.is_empty() {
                    for key_value_pair in key_value_pair_set.split('&') {
                        let mut key_value_split = key_value_pair.splitn(2, "_");
                        if let Some(key) = key_value_split.next() {
                            if let Some(value_string) = key_value_split.next() {
                                if let Ok(value_integer) = u32::from_str_radix(value_string, 16) {
                                    values_mapping.insert(key, value_integer);
                                }
                            }
                        }
                    }
                }
            }
            // Have to perform some bitwise ops on this one so we make it its own variable
            let subsys = values_mapping.get("SUBSYS").unwrap();
            let cc = values_mapping.get("CC").unwrap();

            result.push(PciDevice {
                domain: (win_bus >> 8) & 0xFFFFFF, // Domain is in high 24 bits of SPDRP_BUSNUMBER.
                bus: (win_bus & 0xFF) as u8,       // Bus is in low 8 bits of SPDRP_BUSNUMBER.
                device: ((win_addr >> 16) & 0xFF) as u8, // Device (u8) is in high 16 bits of SPDRP_ADDRESS.
                function: (win_addr & 0xFF) as u8, // Function (u8) is in low 16 bits of SDRP_ADDRESS.
                label: "".to_string(),
                vendor_id: values_mapping.get("VEN").unwrap().to_owned() as u16,
                device_id: values_mapping.get("DEV").unwrap().to_owned() as u16,
                subsys_device_id: (subsys >> 16) as u16, // High 16 bits of SUBSYS.
                subsys_vendor_id: (subsys & 0xFFFF) as u16, // Low 16 bits of SUBSYS.
                class: (cc >> 16) as u8,                 // High 8 bits of CC.
                subclass: ((cc & 0x00FF00) >> 8) as u8,  // Middle 8 bits of CC.
                programming_interface: (cc & 0x0000FF) as u8, // Low 8 bits of CC????? Unsure!
                revision_id: values_mapping.get("REV").unwrap().to_owned() as u8,
                // TODO: Implement all these fields. This is very important!
                // This info is necessary to look up a device's functionality and name.
            });
        }

        SetupDiDestroyDeviceInfoList(device_info)?;
    };

    Ok(result)
}

#[inline]
pub fn _get_pci_by_id(_vendor: u16, _device: u16) -> Result<PciDevice, PciEnumerationError> {
    todo!()
}
