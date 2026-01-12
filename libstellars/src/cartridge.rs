use std::fs;
use std::path::PathBuf;
use std::sync::{RwLock, Weak};
use crate::cartridge::BankSwitching::{Full, Half};
use crate::Stellar;

enum BankSwitching {
    Half,
    Full
}

pub struct Cartridge {
    pub(crate) bus: Option<Weak<RwLock<Stellar>>>,
    bank_switching: BankSwitching
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge {
            bus: None,
            bank_switching: Full
        }
    }

    #[cfg(not(feature = "test-utils"))]
    pub fn load_rom(&mut self, path: PathBuf) {
        match fs::read(path.clone()) {
            Ok(mut data) => {
                let size = data.len();
                let rom_data = if size == 4096 {
                    self.bank_switching = Full;
                    data
                } else if size == 2048 {
                    data.reserve(2048);
                    data.extend_from_within(0..2048);
                    self.bank_switching = Half;
                    data
                } else {
                    panic!("Unknown rom size");
                };

                if let Some(bus_arc) = self.bus.as_ref().and_then(|bus| bus.upgrade()) {
                    let stellar = bus_arc.read().unwrap();
                    stellar.memory.write().unwrap().game_rom = rom_data;

                    stellar.cpu.write().unwrap().init_pc(stellar.read_byte(0xFFFC) as u16 | ((stellar.read_byte(0xFFFD) as u16) << 8));
                }
            }
            Err(err) => {eprintln!("Cannot open ROM {}: {err}", path.display())}
        }
    }
}
