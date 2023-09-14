use crate::backend::common::PciDevice;
use std::fs::*;
use std::io;

use super::common::*;

// ahaha this particular code is by Shibe Drill

#[inline]
pub fn _get_pci_list() -> Result<Vec<PciDevice>, PciEnumerationError> {
    let mut device_list: Vec<PciDevice> = Vec::new();

    /*
    On Linux, PCI device information is stored in /sys/bus/pci/devices/.
    In this directory, there are multiple directories named after PCI addresses in the form of
    0000:00:00.0 where each 0 can be a valid hex digit. These directories contain files that
    hold the information needed to populate the PCI device structure. As follows is the list
    of files and the fields they populate:
        Slot: 
        Label:
        Vendor ID: file 'vendor', 0x prefix
        Device ID: file 'device', 0x prefix
        Subsys Vendor ID: file 'subsystem_device', 0x prefix
        Subsys Device ID: file 'subsystem_vendor', 0x prefix
        Device Class: file 'class', 0x prefix
        Revision ID: file 'revision', 0x prefix
    
    */

    for directory in read_dir("/sys/bus/pci/devices/").unwrap() {
        // Lord forgive me.
        let slot = "Slot".to_string(); // Replace with expression that returns Slot
        let label = "Slot".to_string(); // Replace with expression that returns Label
        let vendor_id = get_pci_device_attribute_u16(&directory, "vendor")?; // Vendor ID
        let device_id = get_pci_device_attribute_u16(&directory, "device")?; // Device ID
        let subsys_device_id = get_pci_device_attribute_u16(&directory, "subsystem_device")?; // Subsystem Device ID
        let subsys_vendor_id = get_pci_device_attribute_u16(&directory, "subsystem_vendor")?; // Subsystem Vendor ID
        let device_class = get_pci_device_attribute_u32(&directory, "class")?; // Device Class
        let revision_id = get_pci_device_attribute_u8(&directory, "revision")?; // Revision ID

        device_list.push(PciDevice {
            slot: slot.to_owned(),
            label: label.to_owned(),
            vendor_id,
            device_id,
            subsys_device_id,
            subsys_vendor_id,
            device_class,
            revision_id,
        })
    }

    // return the list at the end once all the devices are in it.
    Ok(device_list)
}

#[inline]
pub fn _get_pci_by_id(vendor: u16, device: u16) -> Result<PciDevice, PciEnumerationError> {
    todo!()
}

