use std::{cell::RefCell, rc::Rc};

use crate::memory::Memory;

#[derive(Debug)]
struct Registers {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
}

impl Default for Registers {
    fn default() -> Self {
        Self::dmg()
    }
}

impl Registers {
    pub fn dmg() -> Self {
        Registers {
            af: 0x01B0,
            bc: 0x0013,
            de: 0x00D8,
            hl: 0x014D,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }

    pub fn inc_upper(word: u16) -> u16 {
        let upper = Registers::get_lower(word);
        let lower = Registers::get_lower(word);
        (upper + 1 << 8) as u16 | lower as u16
    }

    pub fn dec_upper(word: u16) -> u16 {
        let upper = Registers::get_lower(word);
        let lower = Registers::get_lower(word);
        (upper - 1 << 8) as u16 | lower as u16
    }

    pub fn get_upper(word: u16) -> u8 {
        (word >> 8) as u8
    }

    pub fn get_lower(word: u16) -> u8 {
        (word & 0x00) as u8
    }
}

#[derive(Debug)]
pub struct Cpu {
    registers: Registers,
    memory: Rc<RefCell<Memory>>,
}

impl Cpu {
    pub fn new(mem_bus: Rc<RefCell<Memory>>) -> Self {
        Self {
            registers: Registers::default(),
            memory: mem_bus,
        }
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.memory.borrow().read_byte(self.registers.pc);
        self.registers.pc += 1;

        byte
    }

    fn read_word(&mut self) -> u16 {
        let word = self.memory.borrow().read_word(self.registers.pc);
        self.registers.pc += 2;

        word
    }

    fn mem_write(&self, addr: u16, byte: u8) {
        self.memory.borrow_mut().set_byte(addr, byte);
    }

    fn execute(&mut self, op: u8) {
        match op {
            0x00 => {}
            // LD BC, u16
            0x01 => self.registers.bc = self.read_word(),
            // LD (BC), A
            0x02 => {
                let a = Registers::get_upper(self.registers.af);
                self.mem_write(self.registers.bc, a);
            }
            // INC BC
            0x03 => {
                self.registers.bc += 1;
            }
            // INC B
            0x04 => self.registers.bc = Registers::inc_upper(self.registers.bc),
            // DEC B
            0x05 => self.registers.bc = Registers::dec_upper(self.registers.bc),
            // LD B, u8
            0x06 => {
                let byte = self.read_byte();
                let c = Registers::get_lower(self.registers.bc);
                self.registers.bc = ((byte << 8) as u16) | (c as u16);
            }
            _ => unimplemented!("{:x} is unimplemented", op),
        }
    }
}
