use std::io;
use std::io::Write;
use crate::registers::Registers;

pub struct StellarDebugInfo {
    pub cpu: CpuDebug,
    pub memory: MemoryDebug,
    pub tia: TiaDebug,
}

impl StellarDebugInfo {
    pub fn print_debug(&self) {
        println!("---------------------------------------------------------");
        println!("\t\tAfter opcode execution:\n");
        println!("Executed opcode: 0x{:X} at 0x{:04X}\n", self.cpu.executed_opcode.0, self.cpu.executed_opcode.1);
        println!("Registers:");
        self.cpu.print_registers();
        println!("Flags:");
        self.cpu.print_flags();
        print!("RAM:");
        self.memory.print_ram();
        println!("\n\nTIA:");
        self.tia.print_raster_position();
        println!("---------------------------------------------------------");
    }
}

#[derive(Copy, Clone)]
pub struct CpuDebug {
    pub registers: Registers,
    pub executed_opcode: (u8, u16),
}

impl CpuDebug {
    pub fn print_registers(&self) {
        println!("A: 0x{:02X}\tX: 0x{:02X}\tY: 0x{:02X}\tPC: 0x{:04X}\n", self.registers.acc, self.registers.x, self.registers.y, self.registers.pc);
    }

    pub fn print_flags(&self) {
        println!("N: {}\tV: {}\tB: {}\tD: {}\tI: {}\tZ: {}\tC: {}\n",
                 self.registers.get_n() as u8,
                 self.registers.get_v() as u8,
                 self.registers.get_b() as u8,
                 self.registers.get_d() as u8,
                 self.registers.get_i() as u8,
                 self.registers.get_z() as u8,
                 self.registers.get_c() as u8);
    }
}

#[derive(Copy, Clone)]
pub struct MemoryDebug {
    pub ram: [u8; 0x80]
}

impl MemoryDebug {
    pub fn print_ram(&self) {
        for i in 0x00..0x80 {
            if i % 0x10 == 0 {
                print!("\n0x{:04X}: ", i + 0x80);
            }
            print!("{:02X} ", self.ram[i]);
        }
        io::stdout().flush().unwrap();
    }
}

#[derive(Copy, Clone)]
pub struct TiaDebug {
    pub vsync_enabled: bool,
    pub vblank_enabled: bool,
    pub picture_scanline: u8,
    pub horizontal_counter: u8
}

impl TiaDebug {
    pub fn print_raster_position(&self) {
        println!("Scanline: {}\tHorizontal counter: {}", self.picture_scanline, self.horizontal_counter);
        println!("VSync: {}\tVBlank: {}", self.vsync_enabled, self.vblank_enabled);
    }
}