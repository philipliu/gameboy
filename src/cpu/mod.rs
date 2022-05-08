mod opcode;

const H_BIT_MASK: u16 = 0xFF00;
const L_BIT_MASK: u16 = 0x00FF;

#[derive(Default)]
pub struct Cpu {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub sp: u16,
    pub pc: u16,
    pub flags: u8
}

impl Cpu {
    pub fn new() -> Self {
        Cpu::default()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut cpu = Cpu::new();
        cpu.af = 0xDEAD;
        assert_eq!(cpu.a(), 0xDE);
    }

    #[test]
    fn test_f() {
        let mut cpu = Cpu::new();
        cpu.af = 0xDEAD;
        assert_eq!(cpu.f(), 0xAD);
    }

    #[test]
    fn test_b() {
        let mut cpu = Cpu::new();
        cpu.bc = 0xDEAD;
        assert_eq!(cpu.b(), 0xDE);
    }

    #[test]
    fn test_c() {
        let mut cpu = Cpu::new();
        cpu.bc = 0xDEAD;
        assert_eq!(cpu.c(), 0xAD);
    }

    #[test]
    fn test_d() {
        let mut cpu = Cpu::new();
        cpu.de = 0xDEAD;
        assert_eq!(cpu.d(), 0xDE);
    }

    #[test]
    fn test_e() {
        let mut cpu = Cpu::new();
        cpu.de = 0xDEAD;
        assert_eq!(cpu.e(), 0xAD);
    }

    #[test]
    fn test_h() {
        let mut cpu = Cpu::new();
        cpu.hl = 0xDEAD;
        assert_eq!(cpu.h(), 0xDE);
    }

    #[test]
    fn test_l() {
        let mut cpu = Cpu::new();
        cpu.hl = 0xDEAD;
        assert_eq!(cpu.l(), 0xAD);
    }
}
