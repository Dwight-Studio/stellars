mod opcodes;

use std::cell::RefCell;
use std::rc::Rc;
use serde_json::{Map, Value};
use crate::cpu::opcodes::OPCODES;
use crate::registers::Registers;
use crate::Stellar;

pub struct Cpu {
    pub(crate) bus: Option<Rc<RefCell<Stellar>>>,
    
    registers: Registers,
    cycles: u64,
}

impl Cpu {
    pub fn new(bus: Option<Rc<RefCell<Stellar>>>) -> Self {
        Cpu {
            bus,
            
            registers: Registers::new(),
            cycles: 0,
        }
    }

    pub fn execute(&mut self) {
        let old_cycles = self.cycles;
        let opcode = self.fetch_byte();

        OPCODES[opcode as usize](self);
        
        self.bus.as_ref().unwrap().borrow().tick(self.cycles - old_cycles);
    }

    fn push_stack(&mut self, value: u8) {
        let address = 0x100 + self.registers.sp as u16;
        self.registers.sp -= 1;
        self.cycles += 1;
        
        self.bus.as_ref().unwrap().borrow().write_byte(address, value);
    }

    fn pull_stack(&mut self) -> u8 {
        let address = 0x100 + self.registers.sp as u16;
        self.registers.sp += 1;
        self.cycles += 1;

        self.bus.as_ref().unwrap().borrow().read_byte(address)
    }

    fn fetch_byte(&mut self) -> u8 {
        let data = self.bus.as_ref().unwrap().borrow().read_byte(self.registers.pc);
        self.registers.pc += 1;
        self.cycles += 1;

        data
    }

    fn read_byte(&mut self, address: u16) -> u8 {
        self.cycles += 1;

        self.bus.as_ref().unwrap().borrow().read_byte(address)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.cycles += 1;

        self.bus.as_ref().unwrap().borrow_mut().write_byte(address, value);
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub fn set_registers(&mut self, state: &Map<String, Value>) {
        self.registers.pc = state.get("pc").unwrap().as_u64().unwrap() as u16;
        self.registers.sp = state.get("s").unwrap().as_u64().unwrap() as u8;
        self.registers.acc = state.get("a").unwrap().as_u64().unwrap() as u8;
        self.registers.x = state.get("x").unwrap().as_u64().unwrap() as u8;
        self.registers.y = state.get("y").unwrap().as_u64().unwrap() as u8;
        self.registers.p = state.get("p").unwrap().as_u64().unwrap() as u8;
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub fn check_registers(&self, state: &Map<String, Value>) -> bool {
        let mut flag = true;
        
        flag &= self.registers.pc == state.get("pc").unwrap().as_u64().unwrap() as u16;
        flag &= self.registers.sp == state.get("s").unwrap().as_u64().unwrap() as u8;
        flag &= self.registers.acc == state.get("a").unwrap().as_u64().unwrap() as u8;
        flag &= self.registers.x == state.get("x").unwrap().as_u64().unwrap() as u8;
        flag &= self.registers.y == state.get("y").unwrap().as_u64().unwrap() as u8;
        flag &= self.registers.p == state.get("p").unwrap().as_u64().unwrap() as u8;
        
        flag
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub fn print_registers(&self) {
        println!("PC : {}", self.registers.pc);
        println!("ACC : {}", self.registers.acc);
    }
}