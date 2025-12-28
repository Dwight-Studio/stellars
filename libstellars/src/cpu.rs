mod opcodes;

use crate::cpu::opcodes::OPCODES;
use crate::registers::Registers;
use crate::Stellar;
use serde_json::{Map, Value};
use std::sync::{RwLock, Weak};

pub struct Cpu {
    pub(crate) bus: Option<Weak<RwLock<Stellar>>>,
    
    registers: Registers,
    cycles: u64,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            bus: None,
            
            registers: Registers::new(),
            cycles: 0,
        }
    }

    pub fn execute(&mut self) {
        let old_cycles = self.cycles;
        let opcode = self.fetch_byte();

        OPCODES[opcode as usize](self);

        self.bus.as_ref().unwrap().upgrade().unwrap().read().unwrap().tick(self.cycles - old_cycles);
    }

    pub(crate) fn init_pc(&mut self, pc: u16) {
        self.registers.pc = pc;
    }

    fn push_stack(&mut self, value: u8) {
        let address = 0x100 + self.registers.sp as u16;
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.cycles += 1;

        self.bus.as_ref().unwrap().upgrade().unwrap().read().unwrap().write_byte(address, value);
    }

    fn pull_stack(&mut self) -> u8 {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        self.cycles += 1;
        let address = 0x100 + self.registers.sp as u16;

        self.bus.as_ref().unwrap().upgrade().unwrap().read().unwrap().read_byte(address)
    }

    fn fetch_byte(&mut self) -> u8 {
        let data = self.bus.as_ref().unwrap().upgrade().unwrap().read().unwrap().read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.cycles += 1;

        data
    }

    fn read_byte(&mut self, address: u16) -> u8 {
        self.cycles += 1;

        self.bus.as_ref().unwrap().upgrade().unwrap().read().unwrap().read_byte(address)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.cycles += 1;

        if let Some(bus) = self.bus.as_ref() && let Some(bus_upgrade) = bus.upgrade() && let Ok(bus_w) = bus_upgrade.read() {
            bus_w.write_byte(address, value);
        }
    }

    #[cfg(feature = "test-utils")]
    pub fn set_registers(&mut self, state: &Map<String, Value>) {
        self.registers.pc = state.get("pc").unwrap().as_u64().unwrap() as u16;
        self.registers.sp = state.get("s").unwrap().as_u64().unwrap() as u8;
        self.registers.acc = state.get("a").unwrap().as_u64().unwrap() as u8;
        self.registers.x = state.get("x").unwrap().as_u64().unwrap() as u8;
        self.registers.y = state.get("y").unwrap().as_u64().unwrap() as u8;
        self.registers.p = state.get("p").unwrap().as_u64().unwrap() as u8;
    }

    #[cfg(feature = "test-utils")]
    pub fn check_registers(&self, state: &Map<String, Value>, differences: &mut Vec<String>) -> bool {
        let mut flag = true;

        let expected_pc = state.get("pc").unwrap().as_u64().unwrap() as u16;
        if self.registers.pc != expected_pc {
            differences.push(format!("PC: expected 0x{:04X}, got 0x{:04X}", expected_pc, self.registers.pc));
            flag = false;
        }

        let expected_sp = state.get("s").unwrap().as_u64().unwrap() as u8;
        if self.registers.sp != expected_sp {
            differences.push(format!("SP: expected 0x{:02X}, got 0x{:02X}", expected_sp, self.registers.sp));
            flag = false;
        }

        let expected_acc = state.get("a").unwrap().as_u64().unwrap() as u8;
        if self.registers.acc != expected_acc {
            differences.push(format!("A: expected 0x{:02X}, got 0x{:02X}", expected_acc, self.registers.acc));
            flag = false;
        }

        let expected_x = state.get("x").unwrap().as_u64().unwrap() as u8;
        if self.registers.x != expected_x {
            differences.push(format!("X: expected 0x{:02X}, got 0x{:02X}", expected_x, self.registers.x));
            flag = false;
        }

        let expected_y = state.get("y").unwrap().as_u64().unwrap() as u8;
        if self.registers.y != expected_y {
            differences.push(format!("Y: expected 0x{:02X}, got 0x{:02X}", expected_y, self.registers.y));
            flag = false;
        }

        let expected_p = state.get("p").unwrap().as_u64().unwrap() as u8;
        if self.registers.p != expected_p {
            differences.push(format!("P: expected 0x{:02X}, got 0x{:02X}", expected_p, self.registers.p));
            flag = false;
        }

        flag
    }
}