use crate::backend::common::PciDevice;

#[repr(C)]
pub struct CPciDevice {
    pub slot: *mut ::std::os::raw::c_char,
    pub label: *mut ::std::os::raw::c_char,
    pub vendor_id: u16,
    pub device_id: u16,
    pub subsys_device_id: u16,
    pub subsys_vendor_id: u16,
    pub device_class: u32,
    pub revision_id: u8,
}

#[repr(C)]
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
pub fn _get_pci_list() -> Vec<PciDevice> {
    todo!()
}

#[inline]
pub fn _get_pci_by_id(vendor: u16, device: u16) -> PciDevice {
    todo!()
}
