use crate::mapper::Mapper;

pub struct ThreeE {
    selected_rom_bank: u8,
    selected_ram_bank: u8,
    ram_enabled: bool,
    ram: Vec<u8>,
}

impl ThreeE {
    pub fn new() -> Self {
        Self {
            selected_rom_bank: 0,
            selected_ram_bank: 0,
            ram_enabled: false,
            ram: vec![0u8; 32 * 1024],
        }
    }
}

impl Mapper for ThreeE {
    fn read_rom(&self, rom_data: &[u8], address: usize) -> u8 {
        if address < 0x0800 {
            if self.ram_enabled {
                if address < 0x0400 {
                    let ram_offset = self.selected_ram_bank as usize * 0x400;
                    self.ram[address + ram_offset]
                } else {
                    0
                }
            } else {
                let offset = self.selected_rom_bank as usize * 0x0800;
                if offset + address < rom_data.len() {
                    rom_data[address + offset]
                } else {
                    0
                }
            }
        } else {
            rom_data[(address & 0x07FF) + rom_data.len() - 0x800]
        }
    }

    fn write_ram(&mut self, _rom_data: &mut [u8], address: usize, value: u8) {
        if self.ram_enabled && (0x0400..0x0800).contains(&address) {
            let ram_offset = self.selected_ram_bank as usize * 0x400;
            let ram_address = (address - 0x0400) + ram_offset;
            if ram_address < self.ram.len() {
                self.ram[ram_address] = value;
            }
        }
    }

    fn check_switch(&mut self, address: u16, value: Option<u8>) {
        if let Some(acc) = value {
            if address == 0x3F {
                self.selected_rom_bank = acc;
                self.ram_enabled = false;
            } else if address == 0x3E {
                self.selected_ram_bank = acc;
                self.ram_enabled = true;
            }
        }
    }
}