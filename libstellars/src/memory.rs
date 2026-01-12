use std::fs;
use std::path::PathBuf;
use std::sync::{RwLock, Weak};
use crate::debug::MemoryDebug;
use crate::memory::BankSwitching::{Full, Half, F8};
use crate::Stellar;

#[derive(PartialEq)]
pub enum BankSwitching {
    Half,
    Full,
    F8
}

#[cfg(not(feature = "test-utils"))]
pub struct Memory {
    pub(crate) bus: Option<Weak<RwLock<Stellar>>>,
    pub(crate) ram: [u8; 0x80],
    pub(crate) game_rom: Vec<u8>,

    pub(crate) bank_switching: BankSwitching,
    pub(crate) selected_bank: u16
}

#[cfg(feature = "test-utils")]
pub struct Memory {
    pub(crate) ram: [u8; 0x10000], // 64 KiB pour les tests
}

#[cfg(not(feature = "test-utils"))]
impl Memory {
    pub fn new() -> Self {
        Self {
            bus: None,
            ram: [0x00; 0x80],      // RAM          : Mapped at 0x0080 - 0x00FF
            game_rom: Vec::new(),   // Game ROM Data: Mapped at 0xF000 - 0xFFFF
            bank_switching: Full,
            selected_bank: 0
        }
    }
    
    pub fn get_debug_info(&self) -> MemoryDebug {
        MemoryDebug {
            ram: self.ram
        }
    }

    #[cfg(not(feature = "test-utils"))]
    pub fn load_rom(&mut self, path: PathBuf) {
        match fs::read(path.clone()) {
            Ok(mut data) => {
                let size = data.len();
                let rom_data = if size == 2048 {
                    data.reserve(2048);
                    data.extend_from_within(0..2048);
                    self.bank_switching = Half;
                    data
                } else if size == 4096 {
                    self.bank_switching = Full;
                    data
                } else if size == 8192 {
                    self.selected_bank = 1;
                    self.bank_switching = F8;
                    data
                } else {
                    panic!("Unknown rom size");
                };

                self.game_rom = rom_data;

                if let Some(bus_arc) = self.bus.as_ref().and_then(|bus| bus.upgrade()) {
                    let stellar = bus_arc.read().unwrap();

                    stellar.cpu.write().unwrap().init_pc(self.read_game_rom(0x0FFC) as u16 | ((self.read_game_rom(0x0FFD) as u16) << 8));
                }
            }
            Err(err) => {eprintln!("Cannot open ROM {}: {err}", path.display())}
        }
    }

    pub fn read_game_rom(&self, address: usize) -> u8 {
        if self.bank_switching == F8 {
            if self.selected_bank == 0 {
                self.game_rom[address]
            } else {
                self.game_rom[address + 0x1000]
            }
        } else {
            self.game_rom[address]
        }
    }

    pub fn check_bank_switching(&mut self, address: u16) {
        if self.bank_switching == F8 {
            if address == 0xFFF8 {
                self.selected_bank = 0;
            } else if address == 0xFFF9 {
                self.selected_bank = 1;
            }
        }
    }
}

#[cfg(feature = "test-utils")]
impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0x00; 0x10000],
        }
    }

    pub fn get_debug_info(&self) -> MemoryDebug {
        MemoryDebug {
            ram: [0x00; 0x80]
        }
    }
}