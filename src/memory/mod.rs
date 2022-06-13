const DMG_MEM_SIZE: usize = 1024 * 8;

#[derive(Debug)]
pub struct Memory {
    data: [u8; DMG_MEM_SIZE],
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            data: [0; DMG_MEM_SIZE],
        }
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory::default()
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let start = addr as usize;
        let end = addr as usize + 1;

        ((self.data[start] as u16) << 8 | self.data[end] as u16)
    }

    pub fn set_byte(&mut self, addr: u16, byte: u8) {
        self.data[addr as usize] = byte;
    }
}
