use crate::mapper::Mapper;

pub struct ThreeF {
    selected_bank: u8
}

impl ThreeF {
    pub fn new() -> Self {
        Self { selected_bank: 3 }
    }
}

impl Mapper for ThreeF {
    fn read_rom(&self, rom_data: &[u8], address: usize) -> u8 {
        if address < 0x0800 {
            let offset = self.selected_bank as usize * 0x0800;
            rom_data[address + offset]
        } else {
            rom_data[(address & 0x07FF) + 0x1800]
        }
    }

    fn write_ram(&mut self, _rom_data: &mut [u8], _address: usize, _value: u8) {}

    fn check_switch(&mut self, address: u16, value: Option<u8>) {
        if let Some(acc) = value && address == 0x3F {
            self.selected_bank = acc & 0x03;
        }
    }
}