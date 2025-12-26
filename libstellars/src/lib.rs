use std::cell::RefCell;
use std::rc::Rc;
use serde_json::{Map, Value};
use crate::memory::Memory;
use crate::cpu::Cpu;
use crate::tia::Tia;

mod cpu;
mod registers;
mod memory;
mod tia;

pub const SCREEN_WIDTH: u8 = 160;
pub const SCREEN_HEIGHT: u8 = 192;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

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

    pub fn get_picture_buffer(&self) -> Option<[Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize]> {
        if self.tia.borrow().frame_ready {
            Some(self.tia.borrow().pic_buffer)
        } else {
            None
        }
    }

    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        let data: u8;

        if address <= 0x0D {
            todo!("Input and collision latches")
        } else if (0x0080..=0x00FF).contains(&address) {
            data = self.memory.borrow().ram[(address - 0x80) as usize]
        } else if (0x0100..=0x01FF).contains(&address) {
            data = self.memory.borrow().stack[(address - 0x100) as usize]
        } else if address >= 0xF000 {
            data = self.memory.borrow().game_rom[(address - 0xF000) as usize]
        } else {
            todo!("Logging: warn: Reading at unknown address")
        }

        data
    }

    pub(crate) fn write_byte(&self, address: u16, value: u8) {
        if address <= 0x2C {
            self.tia.borrow_mut().set_write_function(address as u8, value);
        } else if (0x0080..=0x00FF).contains(&address) {
            self.memory.borrow_mut().ram[(address - 0x80) as usize] = value;
        } else if (0x0100..=0x01FF).contains(&address) {
            self.memory.borrow_mut().stack[(address - 0x100) as usize] = value;
        } else {
            todo!("Logging: warn: Writing at unknown address")
        }
    }

    pub(crate) fn tick(&self, cycles: u64) {
        self.tia.borrow_mut().tick(cycles);
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub fn set_initial_state(&self, state: &Map<String, Value>) {
        self.cpu.borrow_mut().set_registers(state);

        let ram_values = state.get("ram").unwrap().as_array().unwrap();
        for value in ram_values {
            let value = value.as_array().unwrap();
            self.write_byte(value.first().unwrap().as_u64().unwrap() as u16, value.get(1).unwrap().as_u64().unwrap() as u8);
        }
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub fn check_final_state(&self, state: &Map<String, Value>) -> bool {
        let mut flag = true;
        flag &= self.cpu.borrow().check_registers(state);

        let ram_values = state.get("ram").unwrap().as_array().unwrap();
        for value in ram_values {
            let value = value.as_array().unwrap();
            flag &= self.read_byte(value.first().unwrap().as_u64().unwrap() as u16) == value.get(1).unwrap().as_u64().unwrap() as u8;
        }

        flag
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub fn run_opcode(&self) {
        self.cpu.borrow().print_registers();
        self.cpu.borrow_mut().execute();
        self.cpu.borrow().print_registers();
    }
}