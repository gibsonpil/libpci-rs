#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum DeviceClass {
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
    Satellite = 0x10,
    Encryption = 0x11,
    DataAcquisition = 0x12,
    Accelerators = 0x13,
    NonEssential = 0x14
}

impl From<DeviceClass> for String {
    fn from(value: DeviceClass) -> Self {
        match value {
            DeviceClass::Undefined => "Undefined",
            DeviceClass::MassStorage => "Mass storage controller",
            DeviceClass::Network => "Network controller",
            DeviceClass::Display => "Display controller",
            DeviceClass::Multimedia => "Multimedia device",
            DeviceClass::Memory => "Memory controller",
            DeviceClass::Bridge => "Bridge device",
            DeviceClass::Communications => "Simple communication controller",
            DeviceClass::Peripheral => "Base system peripheral",
            DeviceClass::Input => "Input device",
            DeviceClass::Docking => "Docking station",
            DeviceClass::Processor => "Processor",
            DeviceClass::Serial => "Serial bus controller",
            DeviceClass::Wireless => "Wireless controller",
            DeviceClass::IntelligentIO => "Intelligent I/O controller",
            DeviceClass::Satellite => "Satellite communication controller",
            DeviceClass::Encryption => "Encryption/decryption controller",
            DeviceClass::DataAcquisition => "Data acquisition and signal processing controller",
            DeviceClass::Accelerators => "Processing accelerator",
            DeviceClass::NonEssential => "Non-essential instrumentation"
        }.to_string()
    }
}

// I like Rust but this kind of thing should not be necessary...
impl From<u8> for DeviceClass {
    fn from(value: u8) -> Self {
        match value {
            0x01 => DeviceClass::MassStorage,
            0x02 => DeviceClass::Network,
            0x03 => DeviceClass::Display,
            0x04 => DeviceClass::Multimedia,
            0x05 => DeviceClass::Memory,
            0x06 => DeviceClass::Bridge,
            0x07 => DeviceClass::Communications,
            0x08 => DeviceClass::Peripheral,
            0x09 => DeviceClass::Input,
            0x0A => DeviceClass::Docking,
            0x0B => DeviceClass::Processor,
            0x0C => DeviceClass::Serial,
            0x0D => DeviceClass::Wireless,
            0x0E => DeviceClass::IntelligentIO,
            0x10 => DeviceClass::Satellite,
            0x11 => DeviceClass::Encryption,
            0x12 => DeviceClass::DataAcquisition,
            0x13 => DeviceClass::Accelerators,
            0x14 => DeviceClass::NonEssential,
            _ => DeviceClass::Undefined,
        }
    }
}