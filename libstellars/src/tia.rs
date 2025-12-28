mod colors;

use crate::tia::colors::NTSC_COLORS;
use crate::{Color, Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::sync::{RwLock, Weak};

#[repr(u8)]
pub enum WriteFunctions {
    Vsync = 0x00,
    Wsync = 0x02,
    Colubk = 0x09,
}
#[repr(u8)]
pub enum ReadFunctions {
    NoneForNow = 0x00,
}

pub struct Tia {
    pub(crate) bus: Option<Weak<RwLock<Stellar>>>,
    pub(crate) frame_ready: bool,
    pub(crate) pic_buffer: [Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],

    write_functions: [u8; 0x2D],
    read_functions: [u8; 0x0E],

    pic_x: u8,
    pic_y: u8,
    wsync_enabled: bool,
    vblank: (bool, u16),
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
            wsync_enabled: false,
            vblank: (true, 0),
            pic_buffer: [Color { r: 0x00, g: 0x00, b: 0x00 }; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
        }
    }

    pub fn set_write_function(&mut self, address: u8, value: u8) {
        self.write_functions[address as usize] = value;

        if address == WriteFunctions::Wsync as u8 {
            self.wsync_enabled = true;
        }
    }

    pub fn get_write_function(&self, address: WriteFunctions) -> u8 {
        self.write_functions[address as usize]
    }

    pub fn tick(&mut self, cycles: u64) {
        for _ in 0..cycles * 3 {
            if (self.get_write_function(WriteFunctions::Vsync) >> 1) & 0x1 == 0x1 {
                self.pic_x = 0x00;
                self.pic_y = 0x00;
                self.wsync_enabled = false;
                self.vblank = (true, 0);
                break;
            }

            if (self.vblank.0) {
                loop {
                    self.vblank.1 += 1;

                    if self.vblank.1 >= 37 * 228 { self.vblank.0 = false; }
                    if self.vblank.1.is_multiple_of(228) { self.wsync_enabled = false; }

                    if (!self.wsync_enabled) { break; }
                }
                break;
            }

            loop {
                if self.pic_x >= 228 {
                    self.pic_x = 0;
                    self.pic_y += 1;
                    self.wsync_enabled = false;
                }

                if self.pic_y >= 192 {
                    self.frame_ready = true;
                    break;
                }

                if self.pic_x >= 68 {
                    self.pic_buffer[self.pic_y as usize * SCREEN_WIDTH as usize + (self.pic_x as usize - 68)] = NTSC_COLORS[self.get_write_function(WriteFunctions::Colubk) as usize];
                }

                self.pic_x += 1;

                if !self.wsync_enabled { break; }
            }
        }
    }
}