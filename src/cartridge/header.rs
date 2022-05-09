/// Cartridge type determines which Memory Bank Controller is used
#[warn(non_camel_case_types)]
#[derive(Debug)]
pub enum CartridgeType {
    ROM_ONLY,
    MBC1,
    MBC1_RAM,
    MBC1_RAM_BATTERY,
    MBC2,
    MBC2_BATTERY,
    ROM_RAM,
    ROM_RAM_BATTERY,
    MMM01,
    MMM01_RAM,
    MMM01_RAM_BATTERY,
    MBC3_TIMER_BATTERY,
    MBC3_TIMER_RAM_BATTERY,
    MBC3,
    MBC3_RAM,
    MBC3_RAM_BATTERY,
    MBC5,
    MBC5_RAM,
    MBC5_RAM_BATTERY,
    MBC5_RUMBLE,
    MBC5_RUMBLE_RAM,
    MBC5_RUMBLE_RAM_BATTERY,
    MBC6,
    MBC7_SENSOR_RUMBLE_RAM_BATTERY,
    POCKET_CAMERA,
    BANDAI_TAMA5,
    HUC3,
    HUC1_RAM_BATTERY,
    Unsupported,
}

impl From<u8> for CartridgeType {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => CartridgeType::ROM_ONLY,
            0x01 => CartridgeType::MBC1,
            0x02 => CartridgeType::MBC1_RAM,
            0x03 => CartridgeType::MBC1_RAM_BATTERY,
            0x05 => CartridgeType::MBC2,
            0x06 => CartridgeType::MBC2_BATTERY,
            0x08 => CartridgeType::ROM_RAM,
            0x09 => CartridgeType::ROM_RAM_BATTERY,
            0x0B => CartridgeType::MMM01,
            0x0C => CartridgeType::MMM01_RAM,
            0x0D => CartridgeType::MMM01_RAM_BATTERY,
            0x0F => CartridgeType::MBC3_TIMER_BATTERY,
            0x10 => CartridgeType::MBC3_TIMER_RAM_BATTERY,
            0x11 => CartridgeType::MBC3,
            0x12 => CartridgeType::MBC3_RAM,
            0x13 => CartridgeType::MBC3_RAM_BATTERY,
            0x19 => CartridgeType::MBC5,
            0x1A => CartridgeType::MBC5_RAM,
            0x1B => CartridgeType::MBC5_RAM_BATTERY,
            0x1C => CartridgeType::MBC5_RUMBLE,
            0x1D => CartridgeType::MBC5_RUMBLE_RAM,
            0x1E => CartridgeType::MBC5_RUMBLE_RAM_BATTERY,
            0x20 => CartridgeType::MBC6,
            0x22 => CartridgeType::MBC7_SENSOR_RUMBLE_RAM_BATTERY,
            0xFC => CartridgeType::POCKET_CAMERA,
            0xFD => CartridgeType::BANDAI_TAMA5,
            0xFE => CartridgeType::HUC3,
            0xFF => CartridgeType::HUC1_RAM_BATTERY,
            _ => CartridgeType::Unsupported,
        }
    }
}

/// Cartridge header metadata
///
/// Reference: <https://gbdev.io/pandocs/The_Cartridge_Header.html#0147---cartridge-type>
/// TODO: parse header info
#[derive(Debug)]
pub struct Header {
    raw_size: usize,
    title: Option<String>,
    manufacturer: Option<String>,
    licensee: Option<String>,
    cgb: u8,
    sgb: u8,
    cartridge_type: CartridgeType,
    rom_size: u8,
    ram_size: u8,
}

pub fn get_header(bytes: &[u8]) -> Header {
    Header {
        raw_size: bytes.len(),
        title: read_string(bytes, 0x134_usize, 0x143_usize),
        manufacturer: read_string(bytes, 0x13F_usize, 0x142_usize),
        licensee: read_string(bytes, 0x144_usize, 0x145_usize),
        cgb: bytes[0x143_usize],
        sgb: bytes[0x146_usize],
        cartridge_type: bytes[0x147].into(),
        rom_size: bytes[0x148],
        ram_size: bytes[0x149],
    }
}

fn read_string(bytes: &[u8], start: usize, end: usize) -> Option<String> {
    let title_bytes = &bytes[start..end];

    match String::from_utf8(title_bytes.to_vec()) {
        Ok(title) => Some(title.trim_end_matches('\0').to_string()),
        _ => None,
    }
}
