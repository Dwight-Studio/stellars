pub struct Memory {
    pub(crate) ram: [u8; 0x80],
    pub(crate) stack: [u8; 0x100],
    pub(crate) game_rom: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0x00; 0x80],      // RAM          : Mapped at 0x0080 - 0x00FF
            stack: [0x00; 0x100],   // Stack        : Mapped at 0x0100 - 0x01FF
            game_rom: Vec::new(),   // Game ROM Data: Mapped at 0xF000 - 0xFFFF
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let data: u8;

        if address >= 0x0080 && address <= 0x00FF {
            data = self.ram[(address - 0x80) as usize]
        } else if address >= 0x0100 && address <= 0x01FF {
            data = self.stack[(address - 0x100) as usize]
        } else if address >= 0xF000 {
            data = self.game_rom[(address - 0xF000) as usize]
        } else {
            todo!("Logging: warn: Reading at unknown address")
        }

        data
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if address >= 0x0080 && address <= 0x00FF {
            self.ram[(address - 0x80) as usize] = value;
        } else if address >= 0x0100 && address <= 0x01FF {
            self.stack[(address - 0x100) as usize] = value;
        } else {
            todo!("Logging: warn: Writing at unknown address")
        }
    }
}