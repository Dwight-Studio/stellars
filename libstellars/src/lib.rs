use std::cell::RefCell;
use std::rc::Rc;
use crate::memory::Memory;
use crate::cpu::Cpu;

mod cpu;
mod registers;
mod memory;

pub const SCREEN_WIDTH: u8 = 160;
pub const SCREEN_HEIGHT: u8 = 192;

pub struct Stellar {
    memory: Rc<RefCell<Memory>>,
    cpu: Cpu
}

impl Stellar {
    pub fn new() -> Self {
        let memory = Rc::new(RefCell::new(Memory::new()));
        let cpu = Cpu::new(memory.clone());

        Self {
            memory,
            cpu,
        }
    }
}