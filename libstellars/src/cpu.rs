mod opcodes;

use crate::cpu::opcodes::OPCODES;
use crate::registers::Registers;

struct Cpu {
    registers: Registers,
    cycles: u64
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            cycles: 0
        }
    }

    pub fn execute(&mut self) {
        let opcode = self.fetch_bytes();

        OPCODES[opcode as usize](self);
    }

    //fixme
    fn write_stack(&mut self, value: u8) {
        let address = 0x100 + self.registers.sp as u16;
        self.registers.sp -= 1;
    }

    //fixme
    fn fetch_bytes(&mut self) -> u8 {
        let data = 0x00;

        self.registers.pc += 1;

        data
    }

    //fixme
    fn read_byte(&self, address: u16) -> u8 {
        let data = 0x00;

        data
    }

    //fixme
    fn write_byte(&mut self, address: u16, value: u8) {

    }
}