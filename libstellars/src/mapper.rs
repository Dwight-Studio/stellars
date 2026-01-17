pub mod half;
pub mod full;
pub mod f8;
pub mod cv;

pub trait Mapper: Send + Sync {
    fn read_rom(&self, rom_data: &[u8], address: usize) -> u8;
    fn write_ram(&mut self, rom_data: &mut [u8], address: usize, value: u8);
    fn check_switch(&mut self, address: u16);
}