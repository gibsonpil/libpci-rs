#[repr(C)]
pub struct CPciDevice {
    pub slot: *mut ::std::os::raw::c_char,
    pub label: *mut ::std::os::raw::c_char,
    pub vendor_id: u16,
    pub device_id: u16,
    pub sub_id: u16,
    pub sub_vendor: u16,
    pub device_class: u16,
    pub revision_id: u8,
}

#[repr(C)]
pub struct CPciDeviceList {
    pub len: usize,
    pub buffer: *mut CPciDevice,
}

extern "C" {
    pub fn get_pci_list() -> CPciDeviceList;
}

extern "C" {
    pub fn get_pci_by_id(vendor: u16, device: u16) -> CPciDevice;
}

