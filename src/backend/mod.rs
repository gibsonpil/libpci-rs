mod bindings;
mod common;
pub use common::PciDevice;

// Get OS implementation.
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::{_get_pci_list, _get_pci_by_id};
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::{_get_pci_list, _get_pci_by_id};
#[cfg(target_os = "macos")]
use bindings::{_get_pci_list, _get_pci_by_id};

fn get_pci_list() -> Vec<PciDevice> {
    return _get_pci_list();
}

fn get_pci_by_id(vendor: u16, device: u16) -> PciDevice {
    return _get_pci_by_id(vendor, device);
}