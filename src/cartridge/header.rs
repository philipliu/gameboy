/// Cartridge header metadata
///
/// Reference: <https://gbdev.io/pandocs/The_Cartridge_Header.html#0147---cartridge-type>
/// TODO: parse header info
#[derive(Debug)]
pub struct Metadata {
    raw_size: usize,
    title: Option<String>,
    manufacturer: Option<String>,
    cbg: u8,
    licensee: Option<String>,
    sbg: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
}

pub fn get_metadata(bytes: &Vec<u8>) -> Metadata {
    Metadata {
        raw_size: bytes.len(),
        title: read_string(bytes, 0x134 as usize, 0x143 as usize),
        manufacturer: read_string(bytes, 0x13F as usize, 0x142 as usize),
        cbg: bytes[0x143 as usize],
        licensee: read_string(bytes, 0x144 as usize, 0x145 as usize),
        sbg: bytes[0x146 as usize],
        cartridge_type: bytes[0x147],
        rom_size: bytes[0x148],
        ram_size: bytes[0x149],
    }
}

fn read_string(bytes: &[u8], start: usize, end: usize) -> Option<String> {
    let title_bytes = &bytes[start..end];

    match String::from_utf8(title_bytes.to_vec()) {
        Ok(title) => Some(title),
        _ => None,
    }
}
