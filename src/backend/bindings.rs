// Copyright (c) 2023 NamedNeon. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors
// may be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::backend::common::{PciDevice, PciEnumerationError};
//use std::ffi::c_void;

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
pub struct CPciDeviceStack {
    pub len: usize,
    pub buffer: *mut CPciDevice,
}

extern "C" {
    fn get_pci_list() -> CPciDeviceStack;
    fn get_pci_by_id(vendor: u16, device: u16) -> CPciDevice;
    fn create_pci_device_stack() -> CPciDeviceStack;
    fn free_pci_device_stack(stack: *mut CPciDeviceStack);
    fn pci_device_stack_push(stack: *mut CPciDeviceStack, device: CPciDevice) -> u32;
    fn pci_device_stack_pop(stack: *mut CPciDeviceStack) -> CPciDevice;
}

#[inline]
pub fn _get_pci_list() -> Result<Vec<PciDevice>, PciEnumerationError> {
    let mut c_pci_stack = unsafe { get_pci_list() };

    unsafe {
        free_pci_device_stack(&mut c_pci_stack);
    }

    todo!()
}

#[inline]
pub fn _get_pci_by_id(vendor: u16, device: u16) -> Result<PciDevice, PciEnumerationError> {
    todo!()
}

// #[cfg(test)]
// mod tests {
//     #[test]
//
// }
