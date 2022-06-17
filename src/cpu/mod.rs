use std::{cell::RefCell, ops::Shl, rc::Rc};

use crate::memory::Memory;

#[derive(Debug)]
pub struct Registers {
    /// Accumulator and flags.
    /// layout: [aaaaaaaaffffchnz]
    /// c: carry
    /// h: half carry (bcd)
    /// n: subtraction carry (bcd)
    /// z: zero flag
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
        (((upper + 1) as u16) << 8) | lower as u16
    }

    pub fn dec_upper(word: u16) -> u16 {
        let upper = Registers::get_lower(word);
        let lower = Registers::get_lower(word);
        (((upper - 1) as u16) << 8) | lower as u16
    }

    pub fn get_upper(word: u16) -> u8 {
        (word >> 8) as u8
    }

    pub fn get_lower(word: u16) -> u8 {
        (word & 0x00FF) as u8
    }

    pub fn is_carry(&self) -> bool {
        (Self::get_lower(self.af) & 0b00001000) > 0
    }

    pub fn concat(upper: u8, lower: u8) -> u16 {
        (upper as u16) << 8 | (lower as u16)
    }
}

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
    memory: Rc<RefCell<Memory>>,
}

impl Cpu {
    pub fn new(mem_bus: Rc<RefCell<Memory>>) -> Self {
        Self {
            registers: Registers::default(),
            memory: mem_bus,
        }
    }

    fn mem_read_byte(&mut self) -> u8 {
        let byte = self.memory.borrow().read_byte(self.registers.pc);
        self.registers.pc += 1;

        byte
    }

    fn mem_read_word(&mut self) -> u16 {
        let word = self.memory.borrow().read_word(self.registers.pc);
        self.registers.pc += 2;

        word
    }

    fn mem_write_byte(&self, addr: u16, byte: u8) {
        self.memory.borrow_mut().set_byte(addr, byte);
    }

    fn mem_write_word(&self, addr: u16, word: u16) {
        self.memory.borrow_mut().set_word(addr, word);
    }

    fn execute(&mut self, op: u8) {
        match op {
            0x00 => {}
            // LD BC, u16
            0x01 => self.registers.bc = self.mem_read_word(),
            // LD (BC), A
            0x02 => {
                let a = Registers::get_upper(self.registers.af);
                self.mem_write_byte(self.registers.bc, a);
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
                let byte = self.mem_read_byte();
                let c = Registers::get_lower(self.registers.bc);
                self.registers.bc = ((byte as u16) << 8) | (c as u16);
            }
            // RLCA
            // rotate A left. carry flag is set to most significant bit
            0x07 => {
                let mut a = Registers::get_upper(self.registers.af);
                let mut f = Registers::get_lower(self.registers.af);
                let carry = a & 0x80;

                a <<= 1;

                if carry > 0 {
                    a |= 0b00000001;
                    f |= 0b00001000;
                }
                self.registers.af = Registers::concat(a, f);
            }
            // LD (u16), SP
            0x08 => {
                let addr = self.mem_read_word();
                self.mem_write_word(addr, self.registers.sp);
            }
            // ADD HL, BC
            0x09 => {}
            _ => unimplemented!("{:x} is unimplemented", op),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rlca_no_carry() {
        // init
        let mut cpu = get_cpu();
        cpu.registers.af = 0x0100;

        // execute
        cpu.execute(0x07);

        // verify
        let a = Registers::get_upper(cpu.registers.af);
        assert_eq!(a, 0x01 << 1);
        assert_eq!(cpu.registers.is_carry(), false);
    }

    #[test]
    fn test_rlca_carry() {
        // init
        let mut cpu = get_cpu();
        cpu.registers.af = 0xFE00;

        // execute
        cpu.execute(0x07);

        // verify
        let a = Registers::get_upper(cpu.registers.af);
        assert_eq!(a, 0xFD);
        assert_eq!(cpu.registers.is_carry(), true);
    }

    fn get_cpu() -> Cpu {
        let memory = Rc::new(RefCell::new(Memory::default()));
        Cpu::new(memory)
    }
}

#[cfg(test)]
mod registers {
    use super::*;

    #[test]
    fn test_registers_get_upper() {
        assert_eq!(Registers::get_upper(0xABCD), 0xAB);
    }

    #[test]
    fn test_registers_get_lower() {
        assert_eq!(Registers::get_lower(0xABCD), 0xCD);
    }

    #[test]
    fn test_concat() {
        let upper = 0xAB;
        let lower = 0xCD;

        assert_eq!(Registers::concat(upper, lower), 0xABCD);
    }
}
