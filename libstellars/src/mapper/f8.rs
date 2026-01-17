use crate::mapper::Mapper;

pub struct F8 {
    selected_bank: u16,
}

impl F8 {
    pub fn new() -> Self {
        Self { selected_bank: 1 }
    }
}

impl Mapper for F8 {
    fn read_rom(&self, rom_data: &[u8], address: usize) -> u8 {
        let offset = if self.selected_bank == 0 { 0 } else { 0x1000 };
        rom_data[address + offset]
    }

    fn write_ram(&mut self, _rom_data: &mut [u8], _address: usize, _value: u8) {}

    fn check_switch(&mut self, address: u16) {
        if address == 0x1FF8 {
            self.selected_bank = 0;
        } else if address == 0x1FF9 {
            self.selected_bank = 1;
        }
    }
}