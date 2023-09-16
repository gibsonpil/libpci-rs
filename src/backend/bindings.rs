use crate::backend::common::{PciDevice, PciEnumerationError};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CPciDevice {
    pub domain: u32,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub subsys_device_id: u16,
    pub subsys_vendor_id: u16,
    pub dev_class: u8,
    pub subclass: u8,
    pub programming_interface: u8,
    pub revision_id: u8,
    pub label: *mut ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CPciDeviceList {
    pub len: usize,
    pub buffer: *mut CPciDevice,
}

extern "C" {
    fn get_pci_list() -> CPciDeviceList;
}

extern "C" {
    fn get_pci_by_id(vendor: u16, device: u16) -> CPciDevice;
}

#[inline]
pub fn _get_pci_list() -> Result<Vec<PciDevice>, PciEnumerationError> {
    todo!()
}

#[inline]
pub fn _get_pci_by_id(vendor: u16, device: u16) -> Result<PciDevice, PciEnumerationError> {
    todo!()
}
