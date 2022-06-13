use std::{cell::RefCell, rc::Rc};

use crate::{cartridge::Cartridge, cpu::Cpu, memory::Memory};

struct Gb {
    cpu: Cpu,
    memory: Rc<RefCell<Memory>>,
}

impl Gb {
    pub fn new() -> Self {
        let memory = Rc::new(RefCell::new(Memory::new()));
        let cpu = Cpu::new(Rc::clone(&memory));

        Self {
            cpu: cpu,
            memory: Rc::clone(&memory),
        }
    }

    pub fn run(&self, cartridge: Cartridge) {
        todo!()
    }
}
