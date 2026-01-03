mod colors;
mod register;

use crate::tia::colors::NTSC_COLORS;
use crate::{Color, Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::sync::{RwLock, Weak};
use std::sync::atomic::Ordering;
use crate::tia::register::Register;

static PM_SIZE: [u8; 4] = [1, 2, 4, 8];
static PM_NUMBER: [&[u8]; 8] = [&[0], &[0, 16], &[0, 32], &[0, 16, 32], &[0, 64], &[0, 8], &[0, 32, 64], &[0, 8, 16, 32]];

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum WORegs {
    Vsync = 0x00,
    Wsync = 0x02,
    Nusiz0 = 0x04,
    Nusiz1 = 0x05,
    Colup0 = 0x06,
    Colup1 = 0x07,
    Colupf = 0x08,
    Colubk = 0x09,
    Ctrlpf = 0x0A,
    Pf0 = 0x0D,
    Pf1 = 0x0E,
    Pf2 = 0x0F,
    Resm0 = 0x12,
    Resm1 = 0x13,
    Enam0 = 0x1D,
    Enam1 = 0x1E,
}
#[repr(u8)]
pub enum RORegs {
    NoneForNow = 0x00,
}

pub struct Tia {
    pub(crate) bus: Option<Weak<RwLock<Stellar>>>,
    pub(crate) pic_buffer: [Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],

    /* Registers */
    wo_regs: [u8; 0x2D],
    ro_regs: [u8; 0x0E],

    /* Internals */
    pic_x: u8,
    pic_y: u8,
    vblank: (bool, u16),
    pf_pixels_per_bit: u8,
    clock_count: u64,
    m0_counter: u8,
    m1_counter: u8,
}

impl Tia {
    pub fn new() -> Tia {
        Self {
            bus: None,
            pic_buffer: [Color { r: 0x00, g: 0x00, b: 0x00 }; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],

            wo_regs: [0x00; 0x2D],
            ro_regs: [0; 0x0E],

            pic_x: 0x0000,
            pic_y: 0x0000,
            vblank: (true, 0),
            pf_pixels_per_bit: (SCREEN_WIDTH as u8 / 2) / 20,
            clock_count: 0,
            m0_counter: 0,
            m1_counter: 0,
        }
    }

    pub fn set_wo_reg(&mut self, address: u8, value: u8) {
        if  address == WORegs::Wsync as u8 {
            self.wo_regs[address as usize] = 0x1;
        } else if address == WORegs::Resm0 as u8 {
            self.m0_counter = 0;
        } else if address == WORegs::Resm1 as u8 {
            self.m1_counter = 158;
        } else {
            self.wo_regs[address as usize] = value;
        }
    }

    pub fn get_wo_reg(&self, address: WORegs) -> Register {
        Register::new(self.wo_regs[address as usize])
    }

    pub fn tick(&mut self, cycles: u64) {
        for _ in 0..cycles * 3 {
            if self.get_wo_reg(WORegs::Vsync).bit(1) {
                self.pic_x = 0x00;
                self.pic_y = 0x00;
                self.wo_regs[WORegs::Wsync as usize] = 0;
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
                    if self.vblank.1.is_multiple_of(228) { self.wo_regs[WORegs::Wsync as usize] = 0; }

                    if !self.get_wo_reg(WORegs::Wsync).bit(0) { break; }
                }
                continue;
            }

            loop {
                if self.pic_x >= 228 {
                    self.pic_x = 0;
                    self.pic_y += 1;
                    self.wo_regs[WORegs::Wsync as usize] = 0;
                }

                if self.pic_x >= 68 && self.pic_y < 192 {
                    if self.get_wo_reg(WORegs::Ctrlpf).bit(2) {
                        self.draw_playfield();
                    } else {
                        self.draw_player(0);
                    }

                    self.m0_counter += 1;
                    self.m1_counter += 1;
                    if self.m0_counter >= SCREEN_WIDTH as u8 { self.m0_counter = 0; }
                    if self.m1_counter >= SCREEN_WIDTH as u8 { self.m1_counter = 0; }
                }

                self.pic_x += 1;
                self.clock_count += 1;

                if !self.get_wo_reg(WORegs::Wsync).bit(0) { break; }
            }
        }

        if self.clock_count >= 59736 {
            self.clock_count -= 59736;
            self.bus.as_ref().unwrap().upgrade().unwrap().read().unwrap().frame_ready.store(true, Ordering::Relaxed);
        }
    }

    fn draw_playfield(&mut self) {
        let rel_pic_x = self.pic_x - 68;
        let pf_register = (self.get_wo_reg(WORegs::Pf0).value.reverse_bits() as u32) << 16 | (self.get_wo_reg(WORegs::Pf1).value as u32) << 8 | (self.get_wo_reg(WORegs::Pf2).value.reverse_bits() as u32);

        if (rel_pic_x < SCREEN_WIDTH as u8 / 2 && (pf_register >> (19 - rel_pic_x / self.pf_pixels_per_bit)) & 0x1 == 1) || // If in first half of screen draw PF pixels as is
            (rel_pic_x >= SCREEN_WIDTH as u8 / 2 && !self.get_wo_reg(WORegs::Ctrlpf).bit(0) && (pf_register >> (19 - (rel_pic_x % (SCREEN_WIDTH as u8 / 2)) / self.pf_pixels_per_bit)) & 0x1 == 1) || // If in second half of screen and in Duplication mode draw the exact same thing as the first half of screen
            (rel_pic_x >= SCREEN_WIDTH as u8 / 2 && self.get_wo_reg(WORegs::Ctrlpf).bit(0) && (pf_register >> ((rel_pic_x % (SCREEN_WIDTH as u8 / 2)) / self.pf_pixels_per_bit)) & 0x1 == 1) { // If in second half of screen and in Reflection mode, draw the mirrored version of the first half of screen

            let mut color: WORegs = WORegs::Colupf;
            if self.get_wo_reg(WORegs::Ctrlpf).bit(1) {
                color = if rel_pic_x < SCREEN_WIDTH as u8 / 2 { WORegs::Colup0 } else { WORegs::Colup1 };
            }
            self.pic_buffer[self.pic_y as usize * SCREEN_WIDTH as usize + (self.pic_x as usize - 68)] = NTSC_COLORS[self.get_wo_reg(color).value as usize];
        } else if self.get_wo_reg(WORegs::Ctrlpf).bit(2) {
            self.draw_ball();
        } else {
            self.draw_background();
        }
    }

    fn draw_missile(&mut self, missile: u8) {
        let missile_enable = if missile == 0 { self.get_wo_reg(WORegs::Enam0) } else { self.get_wo_reg(WORegs::Enam1) };
        let missile_color = if missile == 0 { self.get_wo_reg(WORegs::Colup0) } else { self.get_wo_reg(WORegs::Colup1) };
        let (missile_size, missile_nb) = if missile == 0 { (self.get_wo_reg(WORegs::Nusiz0).value >> 4 & 0x3, self.get_wo_reg(WORegs::Nusiz0).value & 0x7) } else { (self.get_wo_reg(WORegs::Nusiz1).value >> 4 & 0x3, self.get_wo_reg(WORegs::Nusiz1).value & 0x7) };
        let mut triggered = false;

        for trigger in PM_NUMBER[missile_nb as usize] {
            if  (missile == 0 && &self.m0_counter >= trigger && self.m0_counter - trigger < PM_SIZE[missile_size as usize]) ||
                (missile == 1 && &self.m1_counter >= trigger && self.m1_counter - trigger < PM_SIZE[missile_size as usize])
            {
                triggered = true;
                break;
            }
        }

        if missile_enable.bit(1) && triggered {
            self.pic_buffer[self.pic_y as usize * SCREEN_WIDTH as usize + (self.pic_x as usize - 68)] = NTSC_COLORS[missile_color.value as usize];
        } else if missile == 0 {
            self.draw_player(1);
        } else if self.get_wo_reg(WORegs::Ctrlpf).bit(2) {
            self.draw_background();
        } else {
            self.draw_ball();
        }
    }

    fn draw_player(&mut self, player: u8) {
        self.draw_missile(player);
    }

    fn draw_ball(&mut self) {
        if self.get_wo_reg(WORegs::Ctrlpf).bit(2) {
            self.draw_player(0);
        } else {
            self.draw_playfield();
        }
    }

    fn draw_background(&mut self) {
        self.pic_buffer[self.pic_y as usize * SCREEN_WIDTH as usize + (self.pic_x as usize - 68)] = NTSC_COLORS[self.get_wo_reg(WORegs::Colubk).value as usize];
    }
}