use self::header::{Metadata, get_metadata};

mod header;

/// Max ROM size
const MAX_BYTES: usize = 1_572_864;

/// Cartridge errors
#[derive(Debug)]
pub enum CartridgeError {
    IoError(std::io::Error),
    UnexpectedRomSize,
}

impl From<std::io::Error> for CartridgeError {
    fn from(error: std::io::Error) -> Self {
        CartridgeError::IoError(error)
    }
}

/// Reads the contents of a GameBoy ROM
pub fn read(filename: &str) -> Result<Vec<u8>, CartridgeError> {
    let contents = std::fs::read(filename)?;
    Ok(contents)
}

/// A GameBoy cartridge
pub struct Cartridge {
    contents: Vec<u8>,
    pub metadata: Metadata,
}

impl Cartridge {}

impl TryFrom<Vec<u8>> for Cartridge {
    type Error = CartridgeError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, CartridgeError> {
        if bytes.len() > MAX_BYTES {
            Err(CartridgeError::UnexpectedRomSize)
        } else {
            let metadata = get_metadata(&bytes);
            Ok(Cartridge {
                contents: bytes,
                metadata,
            })
        }
    }
}
