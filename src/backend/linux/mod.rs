use crate::backend::common::PciDevice;
use std::fs::*;
use std::io;

use super::common::*;

// ahaha this particular code is by Shibe Drill

#[inline]
pub fn _get_pci_list() -> Vec<PciDevice> {
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
        if let( // If ALL OF THE FOLLOWING are Ok(), (nothing can fail or else the If Let does not succeed)
            Ok(slot),
            Ok(label),
            Ok(vendor_id),
            Ok(device_id),
            Ok(subsys_device_id),
            Ok(subsys_vendor_id),
            Ok(device_class),
            Ok(revision_id),
        ) = ( // as obtained by this here logic below,
            Ok::<&str, &str>("Slot"), // Replace with expression that returns Slot
            Ok::<&str, &str>("Label"), // Replace with expression that returns Label
            get_pci_device_attribute_u16(&directory, "vendor"), // Vendor ID
            get_pci_device_attribute_u16(&directory, "device"), // Device ID
            get_pci_device_attribute_u16(&directory, "subsystem_device"), // Subsystem Device ID
            get_pci_device_attribute_u16(&directory, "subsystem_vendor"), // Subsystem Vendor ID
            get_pci_device_attribute_u32(&directory, "class"), // Device Class
            get_pci_device_attribute_u8(&directory, "revision"), // Revision ID
        ) { // then we push a new PCI device to the list.
            // If ANYTHING fails, the device is not added, and might as well not exist.
            device_list.push(PciDevice {
                slot: slot.to_owned(),
                label: label.to_owned(),
                vendor_id: vendor_id,
                device_id: device_id,
                subsys_device_id: subsys_device_id,
                subsys_vendor_id: subsys_vendor_id,
                device_class: device_class,
                revision_id: revision_id,
            })
        }
    }

    // return the list at the end once all the devices are in it.
    device_list
}

#[inline]
pub fn _get_pci_by_id(vendor: u16, device: u16) -> PciDevice {
    todo!()
}

