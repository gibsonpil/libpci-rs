mod bindings;
mod common;

pub use common::PciDevice;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        use linux::{_get_pci_by_id, _get_pci_list};
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        use self::windows::{_get_pci_by_id, _get_pci_list};
    } else {
        use bindings::{_get_pci_by_id, _get_pci_list};
    }
}

use crate::backend::common::PciEnumerationError;

pub fn get_pci_list() -> Result<Vec<PciDevice>, PciEnumerationError> {
    _get_pci_list()
}

fn get_pci_by_id(vendor: u16, device: u16) -> Result<PciDevice, PciEnumerationError> {
    _get_pci_by_id(vendor, device)
}
