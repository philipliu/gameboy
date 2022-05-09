use crate::{cpu::Cpu, memory::Memory};


struct Gb {
    cpu: Cpu,
    memory: Memory
}

impl Gb {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new()
        }
    }

    pub fn run(&self) {}
}
