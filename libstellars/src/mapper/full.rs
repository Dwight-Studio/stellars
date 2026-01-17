use crate::mapper::Mapper;

pub struct Full;

impl Mapper for Full {
    fn read_rom(&self, rom_data: &[u8], address: usize) -> u8 {
        rom_data[address]
    }

    fn write_ram(&mut self, rom_data: &mut [u8], address: usize, value: u8) {}

    fn check_switch(&mut self, address: u16) {}
}