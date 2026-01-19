use crate::mapper::Mapper;

pub struct F6 {
    selected_bank: u8
}

impl F6 {
    pub fn new() -> Self {
        Self { selected_bank: 3 }
    }
}

impl Mapper for F6 {
    fn read_rom(&self, rom_data: &[u8], address: usize) -> u8 {
        let offset = self.selected_bank as usize * 0x1000;
        rom_data[address + offset]
    }

    fn write_ram(&mut self, _rom_data: &mut [u8], _address: usize, _value: u8) {}

    fn check_switch(&mut self, address: u16, _value: Option<u8>) {
        if address == 0x1FF6 {
            self.selected_bank = 0;
        } else if address == 0x1FF7 {
            self.selected_bank = 1;
        } else if address == 0x1FF8 {
            self.selected_bank = 2;
        } else if address == 0x1FF9 {
            self.selected_bank = 3;
        }
    }
}
