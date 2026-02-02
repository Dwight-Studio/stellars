use std::path::PathBuf;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};
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
mod mapper;

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

        let bus_weak = Arc::downgrade(&bus);
        let bus_read = lock_read(&bus);

        lock_write(&bus_read.memory).bus = Some(bus_weak.clone());
        lock_write(&bus_read.tia).bus = Some(bus_weak.clone());
        lock_write(&bus_read.cpu).bus = Some(bus_weak.clone());

        drop(bus_read);

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
            Some(lock_read(&self.tia).pic_buffer)
        } else {
            None
        }
    }
    
    pub fn unsafe_get_picture_buffer(&self) -> [Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize] {
        lock_read(&self.tia).pic_buffer
    }

    pub fn get_channel_0_samples(&self, number: usize) -> Vec<u8> {
        lock_write(&self.tia).get_channel_0_samples(number)
    }

    pub fn get_channel_1_samples(&self, number: usize) -> Vec<u8> {
        lock_write(&self.tia).get_channel_1_samples(number)
    }

    #[cfg(not(feature = "test-utils"))]
    pub fn load_rom(&self, path: PathBuf) {
        self.reset();
        lock_write(&self.memory).load_rom(path.clone());
        println!("Loaded ROM: {path:?}");
    }
    
    pub fn rom_loaded(&self) -> bool {
        !lock_read(&self.memory).game_rom.is_empty()
    }

    pub fn update_inputs(&self, input: Input, pressed: bool) {
        lock_write(&self.controller).update_inputs(input, pressed);
    }

    pub fn get_debug_info(&self) -> StellarDebugInfo {
        StellarDebugInfo {
            cpu: lock_read(&self.cpu).get_debug_info(),
            memory: lock_read(&self.memory).get_debug_info(),
            tia: lock_read(&self.tia).get_debug_info()
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        if address <= 0x2C {
            lock_read(&self.tia).unsafe_read(address)
        } else {
            self.read_byte(address)
        }
    }

    pub fn use_audio(&self, sample_rate: usize) {
        lock_write(&self.tia).use_audio(sample_rate);
    }

    pub fn reset(&self) {
        lock_write(&self.tia).reset();
        lock_write(&self.cpu).reset();
        lock_write(&self.memory).reset();
        lock_write(&self.pia).reset();
    }

    #[cfg(not(feature = "test-utils"))]
    pub(crate) fn read_byte(&self, mut address: u16) -> u8 {
        address &= 0x1FFF; // CPU 8K Mirrors

        lock_write(&self.memory).check_bank_switching(address, None);

        let data: u8;

        if (address & 0b1_0000_0000_0000) == 0 && (address & 0b1000_0000) == 0 { // TIA Mirrors
            address &= 0x000F;
            if address < 0x8 {
                data = lock_read(&self.tia).read(address);
            } else {
                data = lock_read(&self.controller).read_inputs(address);
            }
        } else if (address & 0b1_0000_0000_0000) == 0 && (address & 0b10_0000_0000) == 0 && (address & 0b1000_0000) != 0 { // PIA RAM Mirrors
            data = lock_read(&self.memory).ram[(address & 0x7F) as usize];
        } else if (address & 0b1_0000_0000_0000) == 0 && (address & 0b10_0000_0000) != 0 && (address & 0b1000_0000) != 0 { // PIA I/O Mirrors
            address &= 0x17;
            if address <= 0x03 {
                data = lock_read(&self.controller).read_inputs(address);
            } else if address & 0x10 == 0 {
                data = lock_write(&self.pia).read(address & 0x01);
            } else {
                data = lock_write(&self.pia).read(address);
            }
        } else if address >= 0x1000 {
            data = lock_read(&self.memory).read_game_rom((address - 0x1000) as usize);
        } else {
            data = 0xFF;
        }

        data
    }

    #[cfg(not(feature = "test-utils"))]
    pub(crate) fn write_byte(&self, mut address: u16, value: u8) {
        address &= 0x1FFF; // CPU 8K Mirrors

        lock_write(&self.memory).check_bank_switching(address, Some(value));

        if (address & 0b1_0000_0000_0000) == 0 && (address & 0b1000_0000) == 0 { // TIA Mirrors
            address &= 0x003F;

            if address <= 0x2C {
                lock_write(&self.tia).set_wo_reg(address as u8, value);
            }
        } else if (address & 0b1_0000_0000_0000) == 0 && (address & 0b10_0000_0000) == 0 && (address & 0b1000_0000) != 0 { // PIA RAM Mirrors
            lock_write(&self.memory).ram[(address & 0x7F) as usize] = value;
        } else if (address & 0b1_0000_0000_0000) == 0 && (address & 0b10_0000_0000) != 0 && (address & 0b1000_0000) != 0 { // PIA I/O Mirrors
            address &= 0x17;
            if address >= 0x14 {
                lock_write(&self.pia).write(address, value);
            }
        } else if address >= 0x1000 {
            lock_write(&self.memory).write_game_ram((address - 0x1000) as usize, value);
        }
    }

    pub(crate) fn tick(&self) {
        lock_write(&self.tia).tick();
        lock_write(&self.pia).tick();
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

pub fn lock_read<T>(l: &'_ RwLock<T>) -> RwLockReadGuard<'_, T> {
    l.read().unwrap_or_else(|e| { eprintln!("Warning: Read lock poisoned!"); e.into_inner() })
}

pub fn lock_write<T>(l: &'_ RwLock<T>) -> RwLockWriteGuard<'_, T> {
    l.write().unwrap_or_else(|e| { eprintln!("Warning: Write lock poisoned!"); e.into_inner() })
}

pub(crate) fn bus_read<R>(bus: &Option<Weak<RwLock<Stellar>>>, f: impl FnOnce(&Stellar) -> R) -> Option<R> {
    // FIXME: The is cool but can hide errors
    let bus = bus.as_ref()?.upgrade()?;
    let guard = bus.read().ok()?;

    Some(f(&guard))
}