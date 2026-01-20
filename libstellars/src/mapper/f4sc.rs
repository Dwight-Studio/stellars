use crate::mapper::Mapper;

pub struct F4SC {
    selected_bank: u8,
}

impl F4SC {
    pub fn new() -> Self {
        Self { selected_bank: 7 }
    }
}

impl Mapper for F4SC {
    fn read_rom(&self, rom_data: &[u8], address: usize) -> u8 {
        let offset = self.selected_bank as usize * 0x1000;
        if (0x80..=0xFF).contains(&address) {
            rom_data[address + offset - 0x80]
        } else {
            rom_data[address + offset]
        }
    }

    fn write_ram(&mut self, rom_data: &mut [u8], address: usize, value: u8) {
        let offset = self.selected_bank as usize * 0x1000;
        if (0x00..=0x7F).contains(&address) {
            rom_data[address + offset] = value;
        }
    }

    fn check_switch(&mut self, address: u16, _value: Option<u8>) {
        if address == 0x1FF4 {
            self.selected_bank = 0;
        } else if address == 0x1FF5 {
            self.selected_bank = 1;
        } else if address == 0x1FF6 {
            self.selected_bank = 2;
        } else if address == 0x1FF7 {
            self.selected_bank = 3;
        } else if address == 0x1FF8 {
            self.selected_bank = 4;
        } else if address == 0x1FF9 {
            self.selected_bank = 5;
        } else if address == 0x1FFA {
            self.selected_bank = 6;
        } else if address == 0x1FFB {
            self.selected_bank = 7;
        }
    }
}