mod colors;
mod register;
mod counter;

use crate::tia::colors::NTSC_COLORS;
use crate::{Color, Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::sync::{RwLock, Weak};
use std::sync::atomic::Ordering;
use crate::debug::{TiaDebug};
use crate::tia::counter::Counter;
use crate::tia::register::Register;

static PMB_SIZE: [u8; 4] = [1, 2, 4, 8];
static PM_NUMBER: [&[u8]; 8] = [&[0], &[0, 16], &[0, 32], &[0, 16, 32], &[0, 64], &[0, 8], &[0, 32, 64], &[0, 8, 16, 32]];

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum WORegs {
    Vsync   = 0x00,
    Wsync   = 0x02,
    Nusiz0  = 0x04,
    Nusiz1  = 0x05,
    Colup0  = 0x06,
    Colup1  = 0x07,
    Colupf  = 0x08,
    Colubk  = 0x09,
    Ctrlpf  = 0x0A,
    Pf0     = 0x0D,
    Pf1     = 0x0E,
    Pf2     = 0x0F,
    Resm0   = 0x12,
    Resm1   = 0x13,
    Resbl   = 0x14,
    Enam0   = 0x1D,
    Enam1   = 0x1E,
    Enabl   = 0x1F,
    Hmm0    = 0x22,
    Hmm1    = 0x23,
    Hmbl    = 0x24,
    Vdelbl  = 0x27,
    Hmove   = 0x2A,
    Hmclr   = 0x2B,
}
/*#[repr(u8)]
pub enum RORegs {
    NoneForNow = 0x00,
}*/

pub struct Tia {
    pub(crate) bus: Option<Weak<RwLock<Stellar>>>,
    pub(crate) pic_buffer: [Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
    tia_debug: TiaDebug,

    /* Registers */
    wo_regs: [u8; 0x2D],
    // ro_regs: [u8; 0x0E],

    /* Internals */
    pic_x: u8,
    pic_y: u8,
    vblank: (bool, u16),
    pf_pixels_per_bit: u8,
    clock_count: u64,

    m0_counter: Counter,
    m1_counter: Counter,
    bl_counter: Counter,
}

impl Tia {
    pub fn new() -> Tia {
        Self {
            bus: None,
            pic_buffer: [Color { r: 0x00, g: 0x00, b: 0x00 }; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
            tia_debug: TiaDebug {
                picture_scanline: 1,
                horizontal_counter: 1,
                vsync_enabled: false,
                vblank_enabled: false
            },

            wo_regs: [0x00; 0x2D],
            // ro_regs: [0; 0x0E],

            pic_x: 0x0000,
            pic_y: 0x0000,
            vblank: (true, 0),
            pf_pixels_per_bit: (SCREEN_WIDTH as u8 / 2) / 20,
            clock_count: 0,
            m0_counter: Counter::new(SCREEN_WIDTH as u8),
            m1_counter: Counter::new(SCREEN_WIDTH as u8),
            bl_counter: Counter::new(SCREEN_WIDTH as u8),
        }
    }

    pub fn set_wo_reg(&mut self, address: u8, value: u8) {
        if  address == WORegs::Wsync as u8 {
            self.wo_regs[address as usize] = 0x1;
        } else if address == WORegs::Resm0 as u8 {
            self.m0_counter.reset();
        } else if address == WORegs::Resm1 as u8 {
            self.m1_counter.reset();
        } else if address == WORegs::Resbl as u8 {
            self.bl_counter.reset();
        } else if address == WORegs::Hmove as u8 {
            self.wo_regs[address as usize] = 8;

            let fbc_val = (self.get_wo_reg(WORegs::Hmm0).value >> 4) ^ 0x8;
            self.m0_counter.increment(fbc_val);

            let fbc_val = (self.get_wo_reg(WORegs::Hmm1).value >> 4) ^ 0x8;
            self.m1_counter.increment(fbc_val);

            let fbc_val = (self.get_wo_reg(WORegs::Hmbl).value >> 4) ^ 0x8;
            self.bl_counter.increment(fbc_val);
        } else if address == WORegs::Hmclr as u8 {
            self.wo_regs[WORegs::Hmm0 as usize] = 0x00;
            self.wo_regs[WORegs::Hmm1 as usize] = 0x00;
            self.wo_regs[WORegs::Hmbl as usize] = 0x00;
        } else {
            self.wo_regs[address as usize] = value;
        }
    }

    pub fn get_wo_reg(&self, address: WORegs) -> Register {
        Register::new(self.wo_regs[address as usize])
    }

    pub fn unsafe_read(&self, address: u16) -> u8 {
        self.wo_regs[address as usize]
    }

    pub fn tick(&mut self) {
        for _ in 0..3 {
            if self.get_wo_reg(WORegs::Vsync).bit(1) {
                self.pic_x = 0x00;
                self.pic_y = 0x00;
                self.tia_debug.picture_scanline = 1;
                self.tia_debug.horizontal_counter = 1;
                self.wo_regs[WORegs::Wsync as usize] = 0;
                self.tia_debug.vsync_enabled = true;
                self.vblank = (true, 0);
                self.clock_count = 684;
                break;
            }
            self.tia_debug.vsync_enabled = false;

            /* VBLANK has been implemented like that because some ROMS didn't use the VBLANK function
               of the TIA. */
            if self.vblank.0 {
                self.tia_debug.vblank_enabled = true;
                loop {
                    self.vblank.1 += 1;
                    self.clock_count += 1;

                    if self.vblank.1 >= 37 * 228 { self.vblank.0 = false; }
                    if self.vblank.1.is_multiple_of(228) { self.wo_regs[WORegs::Wsync as usize] = 0; }

                    if !self.get_wo_reg(WORegs::Wsync).bit(0) { break; }
                }
                continue;
            } else {
                self.tia_debug.vblank_enabled = false;
            }

            loop {
                if self.pic_x >= 228 {
                    self.pic_x = 0;
                    if self.pic_y < SCREEN_HEIGHT as u8 {
                        self.pic_y += 1;
                        self.tia_debug.picture_scanline = self.pic_y + 1;
                    }
                    self.wo_regs[WORegs::Wsync as usize] = 0;
                    self.wo_regs[WORegs::Hmove as usize] = 0;
                }

                if self.pic_x >= 68 + self.get_wo_reg(WORegs::Hmove).value && self.pic_y < 192 {
                    if self.get_wo_reg(WORegs::Ctrlpf).bit(2) {
                        self.draw_playfield();
                    } else {
                        self.draw_player(0);
                    }

                    self.m0_counter.update();
                    self.m1_counter.update();
                    self.bl_counter.update();
                }

                self.pic_x += 1;
                self.tia_debug.horizontal_counter = self.pic_x + 1;
                self.clock_count += 1;

                if !self.get_wo_reg(WORegs::Wsync).bit(0) { break; }
            }
        }

        if self.clock_count >= 59736 {
            self.clock_count -= 59736;
            self.bus.as_ref().unwrap().upgrade().unwrap().read().unwrap().frame_ready.store(true, Ordering::Relaxed);
        }
    }

    pub fn get_debug_info(&self) -> TiaDebug {
        self.tia_debug
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
        let missile_count = if missile == 0 { self.m0_counter.count() } else { self.m1_counter.count() };
        let mut triggered = false;

        for trigger in PM_NUMBER[missile_nb as usize] {
            /* I didn't find the information about why the missiles are shifted of 2 color counts
               (So they never start at the left most of the picture).
               Doing it like that seems wrong and hacky but until I find why, that will work x)

               This might be because a reset is done it two steps:
               - First reset the horizontal counter to 0 (first color clock)
               - Then the horizontal counter is compared with trigger values and set a START signal
                 to indicate that the object should be drawn (second color count)
               - Draw it (first pixel appears on the third pixel)*/
            let trigg = trigger + 2;
            if  missile_count >= trigg && missile_count - trigg < PMB_SIZE[missile_size as usize] {
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
        if self.get_wo_reg(WORegs::Enabl).bit(1) && self.bl_counter.count() < PMB_SIZE[((self.get_wo_reg(WORegs::Ctrlpf).value >> 4) & 0x3) as usize] {
            self.pic_buffer[self.pic_y as usize * SCREEN_WIDTH as usize + (self.pic_x as usize - 68)] = NTSC_COLORS[self.get_wo_reg(WORegs::Colupf).value as usize];
        } else if self.get_wo_reg(WORegs::Ctrlpf).bit(2) {
            self.draw_player(0);
        } else {
            self.draw_playfield();
        }
    }

    fn draw_background(&mut self) {
        self.pic_buffer[self.pic_y as usize * SCREEN_WIDTH as usize + (self.pic_x as usize - 68)] = NTSC_COLORS[self.get_wo_reg(WORegs::Colubk).value as usize];
    }
}