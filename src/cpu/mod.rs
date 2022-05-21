mod instruction;

const H_BIT_MASK: u16 = 0xFF00;
const L_BIT_MASK: u16 = 0x00FF;

#[derive(Default)]
pub struct Registers {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers::default()
    }

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

    pub fn a(&self) -> u8 {
        ((self.af & H_BIT_MASK) >> 8) as u8
    }

    pub fn f(&self) -> u8 {
        (self.af & L_BIT_MASK) as u8
    }

    pub fn b(&self) -> u8 {
        ((self.bc & H_BIT_MASK) >> 8) as u8
    }

    pub fn c(&self) -> u8 {
        (self.bc & L_BIT_MASK) as u8
    }

    pub fn d(&self) -> u8 {
        ((self.de & H_BIT_MASK) >> 8) as u8
    }

    pub fn e(&self) -> u8 {
        (self.de & L_BIT_MASK) as u8
    }

    pub fn h(&self) -> u8 {
        ((self.hl & H_BIT_MASK) >> 8) as u8
    }

    pub fn l(&self) -> u8 {
        (self.hl & L_BIT_MASK) as u8
    }
}

#[derive(Default)]
pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut reg = Registers::new();
        reg.af = 0xDEAD;
        assert_eq!(reg.a(), 0xDE);
    }

    #[test]
    fn test_f() {
        let mut reg = Registers::new();
        reg.af = 0xDEAD;
        assert_eq!(reg.f(), 0xAD);
    }

    #[test]
    fn test_b() {
        let mut reg = Registers::new();
        reg.bc = 0xDEAD;
        assert_eq!(reg.b(), 0xDE);
    }

    #[test]
    fn test_c() {
        let mut reg = Registers::new();
        reg.bc = 0xDEAD;
        assert_eq!(reg.c(), 0xAD);
    }

    #[test]
    fn test_d() {
        let mut reg = Registers::new();
        reg.de = 0xDEAD;
        assert_eq!(reg.d(), 0xDE);
    }

    #[test]
    fn test_e() {
        let mut reg = Registers::new();
        reg.de = 0xDEAD;
        assert_eq!(reg.e(), 0xAD);
    }

    #[test]
    fn test_h() {
        let mut reg = Registers::new();
        reg.hl = 0xDEAD;
        assert_eq!(reg.h(), 0xDE);
    }

    #[test]
    fn test_l() {
        let mut reg = Registers::new();
        reg.hl = 0xDEAD;
        assert_eq!(reg.l(), 0xAD);
    }
}
