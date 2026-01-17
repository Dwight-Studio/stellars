use crate::mapper::Mapper;

pub struct CV;

impl Mapper for CV {
    fn read_rom(&self, rom_data: &[u8], address: usize) -> u8 {
        rom_data[address]
    }

    fn write_ram(&mut self, rom_data: &mut [u8], address: usize, value: u8) {
        if (0x0400..=0x07FF).contains(&address) {
            rom_data[address - 0x400] = value;
        }
    }

    fn check_switch(&mut self, _address: u16) {}
}