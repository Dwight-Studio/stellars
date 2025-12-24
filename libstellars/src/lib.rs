use std::cell::RefCell;
use std::rc::Rc;
use crate::memory::Memory;
use crate::cpu::Cpu;
use crate::tia::Tia;

mod cpu;
mod registers;
mod memory;
mod tia;

pub const SCREEN_WIDTH: u8 = 160;
pub const SCREEN_HEIGHT: u8 = 192;

pub struct Stellar {
    pub(crate) memory: Rc<RefCell<Memory>>,
    tia: Rc<RefCell<Tia>>,
    cpu: Rc<RefCell<Cpu>>,
}

impl Stellar {
    pub fn new() -> Rc<RefCell<Self>> {
        let bus = Rc::new(RefCell::new(Self {
            memory: Rc::new(RefCell::new(Memory::new())),
            tia: Rc::new(RefCell::new(Tia::new(None))),
            cpu: Rc::new(RefCell::new(Cpu::new(None))),
        }));

        bus.borrow().tia.borrow_mut().bus = Some(bus.clone());
        bus.borrow().cpu.borrow_mut().bus = Some(bus.clone());

        bus
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let data: u8;

        if address <= 0x0D {
            todo!("Input and collision latches")
        } else if address >= 0x0080 && address <= 0x00FF {
            data = self.memory.borrow().ram[(address - 0x80) as usize]
        } else if address >= 0x0100 && address <= 0x01FF {
            data = self.memory.borrow().stack[(address - 0x100) as usize]
        } else if address >= 0xF000 {
            data = self.memory.borrow().game_rom[(address - 0xF000) as usize]
        } else {
            todo!("Logging: warn: Reading at unknown address")
        }

        data
    }

    pub fn write_byte(&self, address: u16, value: u8) {
        if address <= 0x2C {
            self.tia.borrow_mut().set_write_function(address as u8, value);
        } else if address >= 0x0080 && address <= 0x00FF {
            self.memory.borrow_mut().ram[(address - 0x80) as usize] = value;
        } else if address >= 0x0100 && address <= 0x01FF {
            self.memory.borrow_mut().stack[(address - 0x100) as usize] = value;
        } else {
            todo!("Logging: warn: Writing at unknown address")
        }
    }

    pub fn tick(&self, cycles: u64) {
        self.tia.borrow_mut().tick(cycles);
    }
}