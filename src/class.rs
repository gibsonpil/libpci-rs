// Copyright (c) 2024 Gibson Pilconis. All rights reserved.
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

include!(concat!(env!("OUT_DIR"), "/pci_classes_phf.rs"));

#[derive(Copy, Clone)]
pub struct PciClassEntry {
    id: u8,
    name: &'static str,
    subclasses: &'static [PciSubclassEntry]
}

#[derive(Copy, Clone)]
pub struct PciSubclassEntry {
    id: u8,
    name: &'static str,
    progs: &'static [PciProgEntry]
}

#[derive(Copy, Clone)]
pub struct PciProgEntry {
    id: u8,
    name: &'static str,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum PciClass {
    Undefined = 0x00,
    MassStorage = 0x01,
    Network = 0x02,
    Display = 0x03,
    Multimedia = 0x04,
    Memory = 0x05,
    Bridge = 0x06,
    Communications = 0x07,
    Peripheral = 0x08,
    Input = 0x09,
    Docking = 0x0A,
    Processor = 0x0B,
    Serial = 0x0C,
    Wireless = 0x0D,
    IntelligentIO = 0x0E,
    Satellite = 0x0F,
    Encryption = 0x10,
    DataAcquisition = 0x11,
    Accelerators = 0x12,
    NonEssential = 0x13,
    Coprocessor = 0x40
}

// I like Rust but this kind of thing should not be necessary...
impl From<u8> for PciClass {
    fn from(value: u8) -> Self {
        match value {
            0x01 => PciClass::MassStorage,
            0x02 => PciClass::Network,
            0x03 => PciClass::Display,
            0x04 => PciClass::Multimedia,
            0x05 => PciClass::Memory,
            0x06 => PciClass::Bridge,
            0x07 => PciClass::Communications,
            0x08 => PciClass::Peripheral,
            0x09 => PciClass::Input,
            0x0A => PciClass::Docking,
            0x0B => PciClass::Processor,
            0x0C => PciClass::Serial,
            0x0D => PciClass::Wireless,
            0x0E => PciClass::IntelligentIO,
            0x10 => PciClass::Satellite,
            0x11 => PciClass::Encryption,
            0x12 => PciClass::DataAcquisition,
            0x13 => PciClass::Accelerators,
            0x14 => PciClass::NonEssential,
            0x40 => PciClass::Coprocessor,
            _ => PciClass::Undefined,
        }
    }
}

impl From<PciClass> for String {
    fn from(value: PciClass) -> Self {
        match value {
            PciClass::Undefined => "Undefined",
            PciClass::MassStorage => "Mass storage controller",
            PciClass::Network => "Network controller",
            PciClass::Display => "Display controller",
            PciClass::Multimedia => "Multimedia device",
            PciClass::Memory => "Memory controller",
            PciClass::Bridge => "Bridge device",
            PciClass::Communications => "Simple communication controller",
            PciClass::Peripheral => "Base system peripheral",
            PciClass::Input => "Input device",
            PciClass::Docking => "Docking station",
            PciClass::Processor => "Processor",
            PciClass::Serial => "Serial bus controller",
            PciClass::Wireless => "Wireless controller",
            PciClass::IntelligentIO => "Intelligent I/O controller",
            PciClass::Satellite => "Satellite communication controller",
            PciClass::Encryption => "Encryption/decryption controller",
            PciClass::DataAcquisition => "Data acquisition and signal processing controller",
            PciClass::Accelerators => "Processing accelerator",
            PciClass::NonEssential => "Non-essential instrumentation",
            PciClass::Coprocessor => "Coprocessor"
        }.to_string()
    }
}

pub fn get_subclass(class_id: u8, subclass_id: u8) -> PciSubclassEntry {
    let class_entry = CLASSES.get(&class_id);
    *class_entry
        .unwrap()
        .subclasses
        .iter()
        .find(|subclass| subclass.id == subclass_id)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::class::get_subclass;

    #[test]
    fn test_get_subclass() {
        let subclass = get_subclass(16, 0);
        assert_eq!(subclass.name, "Network and computing encryption device");
    }
}

