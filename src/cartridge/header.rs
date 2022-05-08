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
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
}

pub fn get_header(bytes: &Vec<u8>) -> Header {
    Header {
        raw_size: bytes.len(),
        title: read_string(bytes, 0x134 as usize, 0x143 as usize),
        manufacturer: read_string(bytes, 0x13F as usize, 0x142 as usize),
        licensee: read_string(bytes, 0x144 as usize, 0x145 as usize),
        cgb: bytes[0x143 as usize],
        sgb: bytes[0x146 as usize],
        cartridge_type: bytes[0x147],
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
