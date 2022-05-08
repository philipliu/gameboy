use crate::cpu::Cpu;


struct Gb {
    cpu: Cpu
}

impl Gb {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new()
        }
    }

    pub fn run(&self) {}
}
