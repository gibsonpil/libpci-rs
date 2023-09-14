use std::mem::size_of;
use crate::backend::common::{PciDevice, PciEnumerationError};
use windows::core::HSTRING;
use windows::Win32::Devices::DeviceAndDriverInstallation::{DIGCF_ALLCLASSES, DIGCF_PRESENT, SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW, SetupDiGetDeviceRegistryPropertyW, SP_DEVINFO_DATA};

impl From<windows::core::Error> for PciEnumerationError {
    fn from(err: windows::core::Error) -> Self {
        // TODO: log more detailed error.
        return PciEnumerationError::OsError;
    }
}

#[inline]
pub fn _get_pci_list() -> Result<Vec<PciDevice>, PciEnumerationError> {
    unsafe {
        let device_info = SetupDiGetClassDevsW(None, &HSTRING::from("PCI"), None, DIGCF_ALLCLASSES | DIGCF_PRESENT)?;

        let mut device_info_data: SP_DEVINFO_DATA = SP_DEVINFO_DATA {
            cbSize: size_of::<SP_DEVINFO_DATA>() as u32,
            ClassGuid: Default::default(),
            DevInst: 0,
            Reserved: 0,
        };

        for i in 0.. {
            if SetupDiEnumDeviceInfo(device_info, i, &mut device_info_data).is_err() {
                // We either don't have any items left or ran into a problem.
                break;
            }

            // SetupDiGetDeviceRegistryPropertyW(device_info, &mut device_info_data, )
        }

        SetupDiDestroyDeviceInfoList(device_info)?;
    };
    todo!()
}

#[inline]
pub fn _get_pci_by_id(vendor: u16, device: u16) -> Result<PciDevice, PciEnumerationError> {
    todo!()
}


