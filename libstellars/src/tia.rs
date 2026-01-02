mod colors;

use crate::tia::colors::NTSC_COLORS;
use crate::{Color, Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::sync::{RwLock, Weak};
use std::sync::atomic::Ordering;

#[repr(u8)]
pub enum WriteFunctions {
    Vsync = 0x00,
    Wsync = 0x02,
    Colup0 = 0x06,
    Colup1 = 0x07,
    Colupf = 0x08,
    Colubk = 0x09,
    Ctrlpf = 0x0A,
    Pf0 = 0x0D,
    Pf1 = 0x0E,
    Pf2 = 0x0F,
}
#[repr(u8)]
pub enum ReadFunctions {
    NoneForNow = 0x00,
}

pub struct Tia {
    pub(crate) bus: Option<Weak<RwLock<Stellar>>>,
    pub(crate) pic_buffer: [Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],

    write_functions: [u8; 0x2D],
    read_functions: [u8; 0x0E],

    pic_x: u8,
    pic_y: u8,
    wsync_enabled: bool,
    vblank: (bool, u16),
    pf_pixels_per_bit: u8,
    clock_count: u64,
}

impl Tia {
    pub fn new() -> Tia {
        Self {
            bus: None,

            write_functions: [0x00; 0x2D],
            read_functions: [0; 0x0E],

            pic_x: 0x0000,
            pic_y: 0x0000,
            wsync_enabled: false,
            vblank: (true, 0),
            pf_pixels_per_bit: (SCREEN_WIDTH as u8 / 2) / 20,
            pic_buffer: [Color { r: 0x00, g: 0x00, b: 0x00 }; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
            clock_count: 0,
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
                self.clock_count = 684;
                break;
            }

            /* VBLANK has been implemented like that because some ROMS didn't use the VBLANK function
               of the TIA. */
            if self.vblank.0 {
                loop {
                    self.vblank.1 += 1;
                    self.clock_count += 1;

                    if self.vblank.1 >= 37 * 228 { self.vblank.0 = false; }
                    if self.vblank.1.is_multiple_of(228) { self.wsync_enabled = false; }

                    if !self.wsync_enabled { break; }
                }
                continue;
            }

            loop {
                if self.pic_x >= 228 {
                    self.pic_x = 0;
                    self.pic_y += 1;
                    self.wsync_enabled = false;
                }

                if self.pic_x >= 68 && self.pic_y < 192 {
                    let rel_pic_x = self.pic_x - 68;
                    let pf_register = (self.get_write_function(WriteFunctions::Pf0).reverse_bits() as u32) << 16 | (self.get_write_function(WriteFunctions::Pf1) as u32) << 8 | (self.get_write_function(WriteFunctions::Pf2).reverse_bits() as u32);

                    // TODO: Bit 3 of Ctrlpf is nt handle for now because of the lack of a sprite implementation
                //           Bit 4 and 5 controls the ball size
                    // TODO: There might be a better way to do the following condition
                    if (rel_pic_x < SCREEN_WIDTH as u8 / 2 && (pf_register >> (19 - rel_pic_x / self.pf_pixels_per_bit)) & 0x1 == 1) || // If in first half of screen draw PF pixels as is
                        (rel_pic_x >= SCREEN_WIDTH as u8 / 2 && self.get_write_function(WriteFunctions::Ctrlpf) & 0x1 == 0 && (pf_register >> (19 - (rel_pic_x % (SCREEN_WIDTH as u8 / 2)) / self.pf_pixels_per_bit)) & 0x1 == 1) || // If in second half of screen and in Duplication mode draw the exact same thing as the first half of screen
                        (rel_pic_x >= SCREEN_WIDTH as u8 / 2 && self.get_write_function(WriteFunctions::Ctrlpf) & 0x1 == 1 && (pf_register >> ((rel_pic_x % (SCREEN_WIDTH as u8 / 2)) / self.pf_pixels_per_bit)) & 0x1 == 1) { // If in second half of screen and in Reflection mode, draw the mirrored version of the first half of screen

                        let mut color: WriteFunctions = WriteFunctions::Colupf;
                        if self.get_write_function(WriteFunctions::Ctrlpf) >> 1 & 0x1 == 1 {
                            color = if rel_pic_x < SCREEN_WIDTH as u8 / 2 { WriteFunctions::Colup0 } else { WriteFunctions::Colup1 };
                        }
                        self.pic_buffer[self.pic_y as usize * SCREEN_WIDTH as usize + (self.pic_x as usize - 68)] = NTSC_COLORS[self.get_write_function(color) as usize];
                    } else {
                        self.pic_buffer[self.pic_y as usize * SCREEN_WIDTH as usize + (self.pic_x as usize - 68)] = NTSC_COLORS[self.get_write_function(WriteFunctions::Colubk) as usize];
                    }
                }

                self.pic_x += 1;
                self.clock_count += 1;

                if !self.wsync_enabled { break; }
            }
        }

        if self.clock_count >= 59736 {
            self.clock_count -= 59736;
            self.bus.as_ref().unwrap().upgrade().unwrap().read().unwrap().frame_ready.store(true, Ordering::Relaxed);
        }
    }
}