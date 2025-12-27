mod colors;

use std::sync::{Arc, RwLock};
use crate::{Color, Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::tia::colors::NTSC_COLORS;

#[repr(u8)]
pub enum WriteFunctions {
    Vsync = 0x00,
    Vblank = 0x01,
    Colubk = 0x09,
}
#[repr(u8)]
pub enum ReadFunctions {
    NoneForNow = 0x00,
}

pub struct Tia {
    pub(crate) bus: Option<Arc<RwLock<Stellar>>>,
    pub(crate) frame_ready: bool,
    pub(crate) pic_buffer: [Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],

    write_functions: [u8; 0x2D],
    read_functions: [u8; 0x0E],

    pic_x: u8,
    pic_y: u8,
}

impl Tia {
    pub fn new() -> Tia {
        Self {
            bus: None,
            frame_ready: false,

            write_functions: [0x00; 0x2D],
            read_functions: [0; 0x0E],

            pic_x: 0x0000,
            pic_y: 0x0000,
            pic_buffer: [Color { r: 0x00, g: 0x00, b: 0x00 }; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
        }
    }

    pub fn set_write_function(&mut self, address: u8, value: u8) {
        self.write_functions[address as usize] = value;
    }

    pub fn get_write_function(&self, address: WriteFunctions) -> u8 {
        self.write_functions[address as usize]
    }

    pub fn tick(&mut self, cycles: u64) {
        for _ in 0..cycles * 3 {
            if (self.get_write_function(WriteFunctions::Vsync) >> 1) & 0x1 == 0x1 ||
                (self.get_write_function(WriteFunctions::Vblank) >> 1) & 0x1 == 0x1 {
                self.frame_ready = true;
                self.pic_x = 0x00;
                self.pic_y = 0x00;
                break;
            }

            if self.pic_x >= 228 {
                self.pic_x = 0;
                self.pic_y += 1;
            }

            if self.pic_y >= 192 {
                break;
            }

            if self.pic_x >= 68 {
                self.pic_buffer[self.pic_x as usize * self.pic_y as usize] = NTSC_COLORS[self.get_write_function(WriteFunctions::Colubk) as usize];
            }

            self.pic_x += 1;
        }
    }
}