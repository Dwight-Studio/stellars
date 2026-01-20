use crate::mapper::Mapper;

pub struct ThreeE {
    selected_bank: u8,
    offset_ram: u8
}

impl ThreeE {
    pub fn new(cutoff: usize) -> Self {
        Self {
            selected_bank: 0,
            offset_ram: (cutoff / 2048) as u8
        }
    }
}

impl Mapper for ThreeE {
    fn read_rom(&self, rom_data: &[u8], address: usize) -> u8 {
        if address < 0x0800 {
            let offset = self.selected_bank as usize * 0x0800;
            rom_data[address + offset]
        } else {
            rom_data[(address & 0x07FF) + rom_data.len() - 0x800]
        }
    }

    fn write_ram(&mut self, rom_data: &mut [u8], address: usize, value: u8) {
        if (0x0400..=0x07FF).contains(&address) {
            rom_data[address - 0x400] = value;
        }
    }

    fn check_switch(&mut self, address: u16, value: Option<u8>) {
        if let Some(acc) = value {
            if address == 0x3E {
                self.selected_bank = acc + self.offset_ram;
            } else if address == 0x3F {
                self.selected_bank = acc;
            }
        }
    }
}