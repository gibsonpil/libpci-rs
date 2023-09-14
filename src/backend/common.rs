pub struct PciDevice {
    pub slot: String,
    pub label: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub sub_id: u16,
    pub sub_vendor: u16,
    pub device_class: u16,
    pub revision_id: u8,
}
