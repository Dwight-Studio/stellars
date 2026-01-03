mod colors;
mod function;

use crate::tia::colors::NTSC_COLORS;
use crate::{Color, Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::sync::{RwLock, Weak};
use std::sync::atomic::Ordering;
use crate::tia::function::Function;

#[repr(u8)]
pub enum WriteFunctions {
    Vsync = 0x00,
    Wsync = 0x02,
    Nusiz0 = 0x04,
    Colup0 = 0x06,
    Colup1 = 0x07,
    Colupf = 0x08,
    Colubk = 0x09,
    Ctrlpf = 0x0A,
    Pf0 = 0x0D,
    Pf1 = 0x0E,
    Pf2 = 0x0F,
    Enam0 = 0x1D,
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

    pub fn get_write_function(&self, address: WriteFunctions) -> Function {
        Function::new(self.write_functions[address as usize])
    }

    pub fn tick(&mut self, cycles: u64) {
        for _ in 0..cycles * 3 {
            if self.get_write_function(WriteFunctions::Vsync).bit(1) {
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
                    if self.get_write_function(WriteFunctions::Ctrlpf).bit(2) {
                        self.draw_playfield();
                    } else {
                        self.draw_player(0);
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

    fn draw_playfield(&mut self) {
        let rel_pic_x = self.pic_x - 68;
        let pf_register = (self.get_write_function(WriteFunctions::Pf0).value.reverse_bits() as u32) << 16 | (self.get_write_function(WriteFunctions::Pf1).value as u32) << 8 | (self.get_write_function(WriteFunctions::Pf2).value.reverse_bits() as u32);

        if (rel_pic_x < SCREEN_WIDTH as u8 / 2 && (pf_register >> (19 - rel_pic_x / self.pf_pixels_per_bit)) & 0x1 == 1) || // If in first half of screen draw PF pixels as is
            (rel_pic_x >= SCREEN_WIDTH as u8 / 2 && !self.get_write_function(WriteFunctions::Ctrlpf).bit(0) && (pf_register >> (19 - (rel_pic_x % (SCREEN_WIDTH as u8 / 2)) / self.pf_pixels_per_bit)) & 0x1 == 1) || // If in second half of screen and in Duplication mode draw the exact same thing as the first half of screen
            (rel_pic_x >= SCREEN_WIDTH as u8 / 2 && self.get_write_function(WriteFunctions::Ctrlpf).bit(0) && (pf_register >> ((rel_pic_x % (SCREEN_WIDTH as u8 / 2)) / self.pf_pixels_per_bit)) & 0x1 == 1) { // If in second half of screen and in Reflection mode, draw the mirrored version of the first half of screen

            let mut color: WriteFunctions = WriteFunctions::Colupf;
            if self.get_write_function(WriteFunctions::Ctrlpf).bit(1) {
                color = if rel_pic_x < SCREEN_WIDTH as u8 / 2 { WriteFunctions::Colup0 } else { WriteFunctions::Colup1 };
            }
            self.pic_buffer[self.pic_y as usize * SCREEN_WIDTH as usize + (self.pic_x as usize - 68)] = NTSC_COLORS[self.get_write_function(color).value as usize];
        } else if self.get_write_function(WriteFunctions::Ctrlpf).bit(2) {
            self.draw_ball();
        } else {
            self.draw_background();
        }
    }

    fn draw_missile(&mut self, missile: u8) {
        if missile == 0 {
            self.draw_player(1);
        } else if self.get_write_function(WriteFunctions::Ctrlpf).bit(2) {
            self.draw_background();
        } else {
            self.draw_ball();
        }
    }

    fn draw_player(&mut self, player: u8) {
        self.draw_missile(player);
    }

    fn draw_ball(&mut self) {
        if self.get_write_function(WriteFunctions::Ctrlpf).bit(2) {
            self.draw_player(0);
        } else {
            self.draw_playfield();
        }
    }

    fn draw_background(&mut self) {
        self.pic_buffer[self.pic_y as usize * SCREEN_WIDTH as usize + (self.pic_x as usize - 68)] = NTSC_COLORS[self.get_write_function(WriteFunctions::Colubk).value as usize];
    }
}