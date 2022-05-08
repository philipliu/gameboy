use cartridge::CartridgeError;
use clap::Parser;

use crate::cartridge::Cartridge;

mod cartridge;
mod cpu;
mod gb;

#[derive(Debug)]
enum GbError {
    Unexpected(CartridgeError)
}

impl From<CartridgeError> for GbError {
    fn from(error: CartridgeError) -> Self {
            GbError::Unexpected(error)
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    rom: String,
}

fn main() -> Result<(), GbError> {
    let args = Args::parse();
    let filename = args.rom;
    println!("Running {}", filename);
    
    let contents = cartridge::read(&filename)?;
    let cartridge: Cartridge = contents.try_into()?;

    println!("metadata: {:#?}", cartridge.header);

    Ok(())
}
