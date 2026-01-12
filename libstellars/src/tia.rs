mod colors;
mod register;
mod object;

use crate::tia::colors::NTSC_COLORS;
use crate::{Color, Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::sync::{RwLock, Weak};
use std::sync::atomic::Ordering;
use crate::debug::{TiaDebug};
use crate::tia::object::Object;
use crate::tia::register::Register;

static PMB_SIZE: [u8; 4] = [1, 2, 4, 8];
static PM_NUMBER: [&[u8]; 8] = [&[0], &[0, 16], &[0, 32], &[0, 16, 32], &[0, 64], &[0, 8], &[0, 32, 64], &[0, 8, 16, 32]];

static NTSC_TIA_AUDIO_CLOCK: u16 = 31399;

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
    Refp0   = 0x0B,
    Refp1   = 0x0C,
    Pf0     = 0x0D,
    Pf1     = 0x0E,
    Pf2     = 0x0F,
    Resp0   = 0x10,
    Resp1   = 0x11,
    Resm0   = 0x12,
    Resm1   = 0x13,
    Resbl   = 0x14,
    Audc0   = 0x15,
    Audc1   = 0x16,
    Audf0   = 0x17,
    Audf1   = 0x18,
    Audv0   = 0x19,
    Audv1   = 0x1A,
    Grp0    = 0x1B,
    Grp1    = 0x1C,
    Enam0   = 0x1D,
    Enam1   = 0x1E,
    Enabl   = 0x1F,
    Hmp0    = 0x20,
    Hmp1    = 0x21,
    Hmm0    = 0x22,
    Hmm1    = 0x23,
    Hmbl    = 0x24,
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
    pic_x: u16,
    pic_y: u16,
    pf_pixels_per_bit: u16,
    clock_count: u64,

    missile0: Object,
    missile1: Object,
    ball: Object,
    player0: Object,
    player1: Object,

    ch1_index: f64,
    prev_ch1_index: f64,
    ch1_poly_4: u8,
    ch1_poly_5: u8,
    ch1_poly_9: u16,
    ch1_square: u8,
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
            pf_pixels_per_bit: (SCREEN_WIDTH as u16 / 2) / 20,
            clock_count: 0,

            missile0: Object::new(),
            missile1: Object::new(),
            ball:     Object::new(),
            player0:  Object::new(),
            player1:  Object::new(),

            ch1_index: 0.0,
            prev_ch1_index: 0.0,
            ch1_square: 0,
            ch1_poly_4: 0xF,
            ch1_poly_5: 0x1F,
            ch1_poly_9: 0x1FF
        }
    }

    pub fn set_wo_reg(&mut self, address: u8, value: u8) {
        if address == WORegs::Vsync as u8 {
            self.wo_regs[address as usize] = value;
            self.pic_y = 0;
            self.pic_x = 0;
            self.tia_debug.picture_scanline = 1;
            self.tia_debug.horizontal_counter = 1;
            self.clock_count = 0;
        } else if  address == WORegs::Wsync as u8 {
            self.wo_regs[address as usize] = 0x1;
        } else if address == WORegs::Resm0 as u8 {
            self.missile0.counter_reset(false);
        } else if address == WORegs::Resm1 as u8 {
            self.missile1.counter_reset(false);
        } else if address == WORegs::Resbl as u8 {
            self.ball.counter_reset(true);
        } else if address == WORegs::Resp0 as u8 {
            self.player0.counter_reset(false);
        } else if address == WORegs::Resp1 as u8 {
            self.player1.counter_reset(false);
        } else if address == WORegs::Hmove as u8 {
            self.wo_regs[address as usize] = 8;

            // TODO: Counter shoudl be incremented based on the clock count instead doing all the
            //       increments all at once
            let fbc_val = (self.get_wo_reg(WORegs::Hmm0).value >> 4) ^ 0x8;
            self.missile0.counter_increment(fbc_val);

            let fbc_val = (self.get_wo_reg(WORegs::Hmm1).value >> 4) ^ 0x8;
            self.missile1.counter_increment(fbc_val);

            let fbc_val = (self.get_wo_reg(WORegs::Hmbl).value >> 4) ^ 0x8;
            self.ball.counter_increment(fbc_val);

            let fbc_val = (self.get_wo_reg(WORegs::Hmp0).value >> 4) ^ 0x8;
            self.player0.counter_increment(fbc_val);

            let fbc_val = (self.get_wo_reg(WORegs::Hmp1).value >> 4) ^ 0x8;
            self.player1.counter_increment(fbc_val);
        } else if address == WORegs::Hmclr as u8 {
            self.wo_regs[WORegs::Hmm0 as usize] = 0x00;
            self.wo_regs[WORegs::Hmm1 as usize] = 0x00;
            self.wo_regs[WORegs::Hmbl as usize] = 0x00;
            self.wo_regs[WORegs::Hmp0 as usize] = 0x00;
            self.wo_regs[WORegs::Hmp1 as usize] = 0x00;
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
            loop {
                if self.pic_x >= 228 {
                    self.pic_x = 0;
                    if self.pic_y <= 262 {
                        self.pic_y += 1;
                        self.tia_debug.picture_scanline = self.pic_y + 1;
                    }
                    self.wo_regs[WORegs::Wsync as usize] = 0;
                    self.wo_regs[WORegs::Hmove as usize] = 0;
                }

                if self.pic_x >= 68 + self.get_wo_reg(WORegs::Hmove).value as u16 && self.pic_y < SCREEN_HEIGHT as u16 {
                    if self.pic_y >= 37 {
                        if self.get_wo_reg(WORegs::Ctrlpf).bit(2) {
                            self.draw_playfield();
                        } else {
                            self.draw_player(0);
                        }
                    } else {
                        self.pic_buffer[self.pic_y as usize * SCREEN_WIDTH as usize + (self.pic_x as usize - 68)] = NTSC_COLORS[0x00];
                    }

                    self.missile0.update();
                    self.missile1.update();
                    self.ball.update();
                    self.player0.update();
                    self.player1.update();
                }

                self.pic_x += 1;
                self.tia_debug.horizontal_counter = self.pic_x + 1;
                self.clock_count += 1;

                if !self.get_wo_reg(WORegs::Wsync).bit(0) { break; }
            }
        }

        if self.clock_count >= 52896 {
            self.clock_count -= 52896;
            self.bus.as_ref().unwrap().upgrade().unwrap().read().unwrap().frame_ready.store(true, Ordering::Relaxed);
        }
    }

    pub fn get_channel_1_samples(&mut self, sample_rate: u64, number: usize) -> Vec<u8> {
        let mut samples: Vec<u8> = Vec::new();

        for _ in 0..number {
            let frequency: f64 = NTSC_TIA_AUDIO_CLOCK as f64 / ((self.get_wo_reg(WORegs::Audf0).value & 0x1F) + 1) as f64;
            let mut incr: f64 = frequency / sample_rate as f64;
            let volume: u8 = self.get_wo_reg(WORegs::Audv0).value;
            let trigger_change = self.prev_ch1_index < 0.5 && self.ch1_index >= 0.5;
            let mut sample = 0;

            if trigger_change {
                self.ch1_square ^= 0x1;
                self.ch1_poly_4 = ((self.ch1_poly_4 >> 1 & 0x1) ^ (self.ch1_poly_4 & 0x1)) << 3 | self.ch1_poly_4 >> 1;
                self.ch1_poly_5 = ((self.ch1_poly_5 >> 2 & 0x1) ^ (self.ch1_poly_5 & 0x1)) << 4 | self.ch1_poly_5 >> 1;
                self.ch1_poly_9 = ((self.ch1_poly_9 >> 4 & 0x1) ^ (self.ch1_poly_9 & 0x1)) << 8 | self.ch1_poly_9 >> 1;
            }

            match self.get_wo_reg(WORegs::Audc0).value {
                0x0 | 0xB => {
                    sample = volume;
                }
                0x1 => {
                    sample = if self.ch1_poly_4 & 0x1 == 0 {0} else {volume};
                }
                0x2 => {
                    incr = (frequency / 15.0) / sample_rate as f64;
                    sample = if self.ch1_poly_4 & 0x1 == 0 {0} else {volume};
                }
                // TODO: Add 0x3
                0x4 | 0x5 => {
                    incr = (frequency / 1.0) / sample_rate as f64;
                    sample = if self.ch1_square & 0x1 == 0 {0} else {volume};
                }
                0x6 | 0xA => {
                    incr = (frequency / 15.5) / sample_rate as f64;
                    sample = if self.ch1_square & 0x1 == 0 {0} else {volume};
                }
                0x7 | 0x9 => {
                    sample = if self.ch1_poly_5 & 0x1 == 0 {0} else {volume};
                }
                0x8 => {
                    sample = if self.ch1_poly_9 & 0x1 == 0 {0} else {volume};
                }
                0xC | 0xD => {
                    incr = (frequency / 3.0) / sample_rate as f64;
                    sample = if self.ch1_square & 0x1 == 0 {0} else {volume};
                }
                0xE => {
                    incr = (frequency / 46.5) / sample_rate as f64;
                    sample = if self.ch1_square & 0x1 == 0 {0} else {volume};
                }
                0xF => {
                    incr = (frequency / 3.0) / sample_rate as f64;
                    sample = if self.ch1_poly_5 & 0x1 == 0 {0} else {volume};
                }
                _ => {}
            }
            samples.push(sample);
            self.prev_ch1_index = self.ch1_index;
            self.ch1_index += incr;
            if self.ch1_index > 1.0 { self.ch1_index -= 1.0 };
        }

        samples
    }

    pub fn get_debug_info(&self) -> TiaDebug {
        self.tia_debug
    }

    fn draw_playfield(&mut self) {
        let rel_pic_x = self.pic_x - 68;
        let pf_register = (self.get_wo_reg(WORegs::Pf0).value.reverse_bits() as u32) << 16 | (self.get_wo_reg(WORegs::Pf1).value as u32) << 8 | (self.get_wo_reg(WORegs::Pf2).value.reverse_bits() as u32);

        if (rel_pic_x < SCREEN_WIDTH as u16 / 2 && (pf_register >> (19 - rel_pic_x / self.pf_pixels_per_bit)) & 0x1 == 1) || // If in first half of screen draw PF pixels as is
            (rel_pic_x >= SCREEN_WIDTH as u16 / 2 && !self.get_wo_reg(WORegs::Ctrlpf).bit(0) && (pf_register >> (19 - (rel_pic_x % (SCREEN_WIDTH as u16 / 2)) / self.pf_pixels_per_bit)) & 0x1 == 1) || // If in second half of screen and in Duplication mode draw the exact same thing as the first half of screen
            (rel_pic_x >= SCREEN_WIDTH as u16 / 2 && self.get_wo_reg(WORegs::Ctrlpf).bit(0) && (pf_register >> ((rel_pic_x % (SCREEN_WIDTH as u16 / 2)) / self.pf_pixels_per_bit)) & 0x1 == 1) { // If in second half of screen and in Reflection mode, draw the mirrored version of the first half of screen

            let mut color: WORegs = WORegs::Colupf;
            if self.get_wo_reg(WORegs::Ctrlpf).bit(1) {
                color = if rel_pic_x < SCREEN_WIDTH as u16 / 2 { WORegs::Colup0 } else { WORegs::Colup1 };
            }
            self.pic_buffer[self.pic_y as usize * SCREEN_WIDTH as usize + (self.pic_x as usize - 68)] = NTSC_COLORS[self.get_wo_reg(color).value as usize];
        } else if self.get_wo_reg(WORegs::Ctrlpf).bit(2) {
            self.draw_ball();
        } else {
            self.draw_background();
        }
    }

    fn draw_missile(&mut self, missile: u8) {
        let missile_can_draw            = if missile == 0 { self.missile0.can_draw() } else { self.missile1.can_draw() };
        let missile_enable              = if missile == 0 { self.get_wo_reg(WORegs::Enam0) } else { self.get_wo_reg(WORegs::Enam1) };
        let missile_color               = if missile == 0 { self.get_wo_reg(WORegs::Colup0) } else { self.get_wo_reg(WORegs::Colup1) };
        let (missile_size, missile_nb)  = if missile == 0 { (self.get_wo_reg(WORegs::Nusiz0).value >> 4 & 0x3, self.get_wo_reg(WORegs::Nusiz0).value & 0x7) } else { (self.get_wo_reg(WORegs::Nusiz1).value >> 4 & 0x3, self.get_wo_reg(WORegs::Nusiz1).value & 0x7) };
        let missile_count               = if missile == 0 { self.missile0.count() } else { self.missile1.count() };
        let mut triggered               = false;


        for trigger in PM_NUMBER[missile_nb as usize] {
            let trigg = trigger + 4;
            if  missile_count >= trigg && missile_count - trigg < PMB_SIZE[missile_size as usize] {
                triggered = true;
                break;
            }
        }

        if missile_can_draw && missile_enable.bit(1) && triggered {
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
        let player_can_draw = if player == 0 { self.player0.can_draw() } else { self.player1.can_draw() };
        let player_color    = if player == 0 { self.get_wo_reg(WORegs::Colup0) } else { self.get_wo_reg(WORegs::Colup1) };
        let player_nb       = if player == 0 { self.get_wo_reg(WORegs::Nusiz0).value & 0x7 } else { self.get_wo_reg(WORegs::Nusiz1).value & 0x7 };
        let player_count    = if player == 0 { self.player0.count() } else { self.player1.count() };
        let mut player_graphic = if player == 0 { self.get_wo_reg(WORegs::Grp0) } else { self.get_wo_reg(WORegs::Grp1) };
        let player_refl     = if player == 0 { self.get_wo_reg(WORegs::Refp0) } else { self.get_wo_reg(WORegs::Refp1) };
        let mut triggered   = (false, false);

        if player_refl.bit(3) { player_graphic = player_graphic.reverse_bits(); }

        for trigger in PM_NUMBER[player_nb as usize] {
            let trigg = trigger + 5;
            if  player_count >= trigg && player_count - trigg < 8 {
                triggered = (true, player_graphic.bit(7 - (player_count - trigg)));
                break;
            }
        }

        if player_can_draw && player_graphic.value != 0x00 && triggered.0 && triggered.1 {
            self.pic_buffer[self.pic_y as usize * SCREEN_WIDTH as usize + (self.pic_x as usize - 68)] = NTSC_COLORS[player_color.value as usize];
        } else {
            self.draw_missile(player);
        }
    }

    fn draw_ball(&mut self) {
        if self.get_wo_reg(WORegs::Enabl).bit(1) && self.ball.count() < PMB_SIZE[((self.get_wo_reg(WORegs::Ctrlpf).value >> 4) & 0x3) as usize] {
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