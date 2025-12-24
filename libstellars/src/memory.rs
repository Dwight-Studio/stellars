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
}