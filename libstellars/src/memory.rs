use crate::debug::MemoryDebug;

#[cfg(not(feature = "test-utils"))]
pub struct Memory {
    pub(crate) ram: [u8; 0x80],
    pub(crate) game_rom: Vec<u8>,
}

#[cfg(feature = "test-utils")]
pub struct Memory {
    pub(crate) ram: [u8; 0x10000], // 64 KiB pour les tests
}

#[cfg(not(feature = "test-utils"))]
impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0x00; 0x80],      // RAM          : Mapped at 0x0080 - 0x00FF
            game_rom: Vec::new(),   // Game ROM Data: Mapped at 0xF000 - 0xFFFF
        }
    }
    
    pub fn get_debug_info(&self) -> MemoryDebug {
        MemoryDebug {
            ram: self.ram
        }
    }
}

#[cfg(feature = "test-utils")]
impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0x00; 0x10000],
        }
    }

    pub fn get_debug_info(&self) -> MemoryDebug {
        MemoryDebug {
            ram: [0x00; 0x80]
        }
    }
}