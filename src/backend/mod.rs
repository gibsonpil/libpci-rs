mod bindings;
mod common;
pub use common::PciDevice;

// Get OS implementation.
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::{_get_pci_by_id, _get_pci_list};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use self::windows::{_get_pci_by_id, _get_pci_list};

#[cfg(target_os = "macos")]
use bindings::{_get_pci_by_id, _get_pci_list};

use crate::backend::common::PciEnumerationError;

pub fn get_pci_list() -> Result<Vec<PciDevice>, PciEnumerationError> {
    _get_pci_list()
}

fn get_pci_by_id(vendor: u16, device: u16) -> Result<PciDevice, PciEnumerationError> {
    _get_pci_by_id(vendor, device)
}
