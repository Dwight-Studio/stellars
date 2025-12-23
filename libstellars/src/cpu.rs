mod opcodes;

use std::cell::RefCell;
use std::rc::Rc;
use crate::cpu::opcodes::OPCODES;
use crate::memory::Memory;
use crate::registers::Registers;

pub struct Cpu {
    mem: Rc<RefCell<Memory>>,
    
    registers: Registers,
    cycles: u64
}

impl Cpu {
    pub fn new(mem: Rc<RefCell<Memory>>) -> Self {
        Cpu {
            mem,
            
            registers: Registers::new(),
            cycles: 0
        }
    }

    pub fn execute(&mut self) {
        let opcode = self.fetch_byte();

        OPCODES[opcode as usize](self);
    }

    fn write_stack(&mut self, value: u8) {
        let address = 0x100 + self.registers.sp as u16;
        self.registers.sp -= 1;
        
        self.mem.borrow_mut().write_byte(address, value);
    }

    fn read_stack(&mut self) -> u8 {
        let address = 0x100 + self.registers.sp as u16;
        self.registers.sp += 1;

        self.mem.borrow().read_byte(address)
    }

    fn fetch_byte(&mut self) -> u8 {
        let data;

        data = self.mem.borrow().read_byte(self.registers.pc);
        self.registers.pc += 1;

        data
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.mem.borrow().read_byte(address)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.mem.borrow_mut().write_byte(address, value);
    }
}