//! Instruction set decoding
//! source: <https://izik1.github.io/gbops/index.html>

pub struct Instruction {
    opcode: Opcode,
    addressing_mode: AddressingMode
}

#[allow(non_camel_case_types)]
pub enum Opcode {
    NOP,
    LD
}

pub enum AddressingMode {
    Implied
}

impl Instruction {
    fn new(opcode: Opcode, addressing_mode: AddressingMode) -> Self {
        Self {
            opcode,
            addressing_mode
        }
    }
    
    pub fn decode(byte: u8) -> Instruction {
        match byte {
            0x00 => Instruction::new(Opcode::NOP, AddressingMode::Implied),
            _ => todo!()
        }
    }
}