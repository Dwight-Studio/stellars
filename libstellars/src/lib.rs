use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::controller::{Controller, Input};
use crate::memory::Memory;
use crate::cpu::Cpu;
use crate::tia::Tia;

#[cfg(feature = "test-utils")]
use serde_json::{Map, Value};
use crate::debug::StellarDebugInfo;
use crate::pia::Pia;

mod cpu;
mod registers;
mod memory;
mod tia;
pub mod controller;
mod pia;
mod debug;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 262;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Stellar {
    pub(crate) memory: Arc<RwLock<Memory>>,
    tia: Arc<RwLock<Tia>>,
    cpu: Arc<RwLock<Cpu>>,
    controller: Arc<RwLock<Controller>>,
    pia: Arc<RwLock<Pia>>,
    
    frame_ready: AtomicBool,
}

impl Stellar {
    pub fn new() -> Arc<RwLock<Self>> {
        let bus = Arc::new(RwLock::new(Self {
            memory: Arc::new(RwLock::new(Memory::new())),
            tia: Arc::new(RwLock::new(Tia::new())),
            cpu: Arc::new(RwLock::new(Cpu::new())),
            controller: Arc::new(RwLock::new(Controller::new())),
            pia: Arc::new(RwLock::new(Pia::new())),
            
            frame_ready: AtomicBool::new(false),
        }));

        bus.read().unwrap().tia.write().unwrap().bus = Some(Arc::downgrade(&bus));
        bus.read().unwrap().cpu.write().unwrap().bus = Some(Arc::downgrade(&bus));

        bus
    }

    pub fn execute(&self) {
        if let Ok(mut cpu) = self.cpu.write() {
            cpu.execute();
        }
    }

    pub fn get_picture_buffer(&self) -> Option<[Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize]> {
        if self.frame_ready.load(Ordering::Relaxed) {
            self.frame_ready.store(false, Ordering::Relaxed);
            Some(self.tia.read().unwrap().pic_buffer)
        } else {
            None
        }
    }
    
    pub fn unsafe_get_picture_buffer(&self) -> [Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize] {
        self.tia.read().unwrap().pic_buffer
    }

    pub fn get_channel_1_samples(&self, sample_rate: u64, number: usize) -> Vec<u8> {
        self.tia.write().unwrap().get_channel_1_samples(sample_rate, number)
    }

    #[cfg(not(feature = "test-utils"))]
    pub fn load_rom(&self, path: PathBuf) {
        match fs::read(path.clone()) {
            Ok(mut data) => {
                let size = data.len();
                if size == 4096 {
                    self.memory.write().unwrap().game_rom = data;
                } else if size == 2048 {
                    data.reserve(2048);
                    data.extend_from_within(0..2048);
                    self.memory.write().unwrap().game_rom = data;
                } else {
                    panic!("Unknown rom size");
                }
                self.cpu.write().unwrap().init_pc(self.read_byte(0xFFFC) as u16 | ((self.read_byte(0xFFFD) as u16) << 8));
            }
            Err(err) => {eprintln!("Cannot open ROM {}: {err}", path.display())}
        }
    }

    pub fn update_inputs(&self, input: Input, pressed: bool) {
        self.controller.write().unwrap().update_inputs(input, pressed)
    }

    pub fn get_debug_info(&self) -> StellarDebugInfo {
        StellarDebugInfo {
            cpu: self.cpu.read().unwrap().get_debug_info(),
            memory: self.memory.read().unwrap().get_debug_info(),
            tia: self.tia.read().unwrap().get_debug_info()
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        if address <= 0x2C {
            self.tia.read().unwrap().unsafe_read(address)
        } else {
            self.read_byte(address)
        }
    }

    #[cfg(not(feature = "test-utils"))]
    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        let data: u8;

        if address <= 0x0B {
            /*todo!("Input and collision latches")*/
            data = 0xFF;
        } else if (0x0080..=0x00FF).contains(&address) {
            data = self.memory.read().unwrap().ram[(address - 0x80) as usize]
        } else if (0x0180..=0x01FF).contains(&address) {
            data = self.memory.read().unwrap().ram[(address - 0x180) as usize]
        } else if (0x0280..=0x0283).contains(&address) || (0x000C..=0x000D).contains(&address) {
            data = self.controller.read().unwrap().read_inputs(address);
        } else if (0x0284..=0x0285).contains(&address) || (0x0294..=0x0297).contains(&address) {
            data = self.pia.write().unwrap().read(address);
        } else if address >= 0xF000 {
            data = self.memory.read().unwrap().game_rom[(address - 0xF000) as usize]
        } else {
            data = 0xFF;
            // TODO: Reading at unknown address
        }

        data
    }

    #[cfg(not(feature = "test-utils"))]
    pub(crate) fn write_byte(&self, address: u16, value: u8) {
        if address <= 0x2C {
            self.tia.write().unwrap().set_wo_reg(address as u8, value);
        } else if (0x0080..=0x00FF).contains(&address) {
            self.memory.write().unwrap().ram[(address - 0x80) as usize] = value;
        } else if (0x0180..=0x01FF).contains(&address) {
            self.memory.write().unwrap().ram[(address - 0x180) as usize] = value;
        } else if (0x0294..=0x0297).contains(&address) {
            self.pia.write().unwrap().write(address, value);
        }
    }

    pub(crate) fn tick(&self) {
        self.tia.write().unwrap().tick();
        self.pia.write().unwrap().tick();
    }

    #[cfg(feature = "test-utils")]
    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        self.memory.read().unwrap().ram[address as usize]
    }

    #[cfg(feature = "test-utils")]
    pub(crate) fn write_byte(&self, address: u16, value: u8) {
        self.memory.write().unwrap().ram[address as usize] = value;
    }

    #[cfg(feature = "test-utils")]
    pub fn set_initial_state(&self, state: &Map<String, Value>) {
        self.cpu.write().unwrap().set_registers(state);

        let ram_values = state.get("ram").unwrap().as_array().unwrap();
        for value in ram_values {
            let value = value.as_array().unwrap();
            self.write_byte(value.first().unwrap().as_u64().unwrap() as u16, value.get(1).unwrap().as_u64().unwrap() as u8);
        }
    }

    #[cfg(feature = "test-utils")]
    pub fn check_final_state(&self, state: &Map<String, Value>, cycles_value: Option<&Vec<Value>>) -> (bool, Vec<String>) {
        let mut differences = Vec::new();
        let mut flag;

        flag = self.cpu.read().unwrap().check_registers(state, &mut differences);

        let ram_values = state.get("ram").unwrap().as_array().unwrap();
        for value in ram_values {
            let value = value.as_array().unwrap();
            let address = value.first().unwrap().as_u64().unwrap() as u16;
            let expected = value.get(1).unwrap().as_u64().unwrap() as u8;
            let actual = self.read_byte(address);

            if actual != expected {
                differences.push(format!("RAM[0x{:04X}]: expected 0x{:02X}, got 0x{:02X}", address, expected, actual));
                flag = false;
            }
        }

        if let Some(cycles_values) = cycles_value {
            let cycles_info = &self.cpu.read().unwrap().cycles_info;

            for (expected, received) in cycles_values.iter().zip(cycles_info.iter()) {
                let cycle = expected.as_array().unwrap();
                let expected_address = cycle.first().unwrap().as_u64().unwrap() as u16;
                let expected_value = cycle.get(1).unwrap().as_u64().unwrap() as u8;
                let expected_mode = cycle.get(2).unwrap().as_str().unwrap();

                let (received_address, received_value, received_mode) = received;

                if expected_address != *received_address {
                    differences.push(format!("Cycle address: expected 0x{:04X}, got 0x{:04X}", expected_address, received_address));
                    flag = false;
                }
                if expected_value != *received_value {
                    differences.push(format!("Cycle value: expected 0x{:02X}, got 0x{:02X}", expected_value, received_value));
                    flag = false;
                }
                if expected_mode != *received_mode {
                    differences.push(format!("Cycle mode: expected {}, got {}", expected_mode, received_mode));
                    flag = false;
                }
            }

            if cycles_info.len() != cycles_values.len() {
                differences.push(format!("Cycle length: expected {}, got {}", cycles_values.len(), cycles_info.len()));
                flag = false;
            }

            if !flag {
                eprintln!("expected: {:?}", cycles_values);
                eprintln!("received: {:?}", cycles_info);
            }
        }

        (flag, differences)
    }

    #[cfg(feature = "test-utils")]
    pub fn run_opcode(&self) {
        self.cpu.write().unwrap().execute();
    }
}