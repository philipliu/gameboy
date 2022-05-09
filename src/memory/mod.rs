const DMG_MEM_SIZE: usize = 1024 * 8;

pub struct Memory {
    data: [u8; DMG_MEM_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [0; DMG_MEM_SIZE],
        }
    }
}
