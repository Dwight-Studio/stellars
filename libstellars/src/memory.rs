use std::{fs, io};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{RwLock, Weak};
use crate::debug::MemoryDebug;
use crate::mapper::cv::CV;
use crate::mapper::f8::F8;
use crate::mapper::f6::F6;
use crate::mapper::f4::F4;
use crate::mapper::Mapper;
use crate::mapper::full::Full;
use crate::mapper::half::Half;
use crate::mapper::threef::ThreeF;
use crate::Stellar;

#[cfg(not(feature = "test-utils"))]
pub struct Memory {
    pub(crate) bus: Option<Weak<RwLock<Stellar>>>,
    pub(crate) ram: [u8; 0x80],
    pub(crate) game_rom: Vec<u8>,
    pub(crate) mapper: Box<dyn Mapper>
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
            mapper: Box::new(Full)
        }
    }

    pub fn get_debug_info(&self) -> MemoryDebug {
        MemoryDebug {
            ram: self.ram
        }
    }

    #[cfg(not(feature = "test-utils"))]
    //fixme: handle cartridge banking method better
    pub fn load_rom(&mut self, path: PathBuf) {
        match fs::read(path.clone()) {
            Ok(mut data) => {
                let size = data.len();
                let rom_data = if size == 2048 {
                    data.reserve(2048);
                    data.extend_from_within(0..2048);

                    println!("Choose cartridge banking :");
                    println!("1 - Classic 2k");
                    println!("2 - CV only for Commavid");
                    let mut input: String = String::new();
                    loop {
                        print!("> ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input).unwrap();
                        if let Ok(value) = input.trim().parse::<u32>() {
                            if value == 1 {
                                self.mapper = Box::new(Half);
                                break
                            } else if value == 2 {
                                self.mapper = Box::new(CV);
                                break
                            }
                        }
                    }

                    data
                } else if size == 4096 {
                    self.mapper = Box::new(Full);
                    data
                } else if size == 8192 {
                    println!("Choose cartridge banking :");
                    println!("1 - F8");
                    println!("2 - 3F");
                    let mut input: String = String::new();
                    loop {
                        print!("> ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input).unwrap();
                        if let Ok(value) = input.trim().parse::<u32>() {
                            if value == 1 {
                                self.mapper = Box::new(F8::new());
                                break
                            } else if value == 2 {
                                self.mapper = Box::new(ThreeF::new());
                                break
                            }
                        }
                    }
                    data
                } else if size == 16384 {
                    self.mapper = Box::new(F6::new());
                    data
                } else if size == 32768 {
                    self.mapper = Box::new(F4::new());
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
        self.mapper.read_rom(&self.game_rom, address)
    }

    pub fn write_game_ram(&mut self, address: usize, data: u8) {
        self.mapper.write_ram(&mut self.game_rom, address, data);
    }

    pub fn check_bank_switching(&mut self, address: u16, value: Option<u8>) {
        self.mapper.check_switch(address, value);
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