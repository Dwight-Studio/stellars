const NTSC_TIA_AUDIO_CLOCK: f64 = 31399.5;

pub struct AudioChannel {
    audf: u8,
    audv: u8,
    audc: u8,
    sample_rate: usize,

    index: f64,
    prev_index: f64,
    square: u8,
    poly_4: u8,
    poly_4_3: u8,
    poly_5: u8,
    poly_5_3: u8,
    poly_9: u16,
}

impl AudioChannel {
    pub fn new(sample_rate: usize) -> Self {
        Self {
            audf: 0x00,
            audv: 0x00,
            audc: 0x00,
            sample_rate,

            index: 0.0,
            prev_index: 0.0,
            square: 0,
            poly_4: 0xF,
            poly_4_3: 0xF,
            poly_5: 0x1F,
            poly_5_3: 0x1F,
            poly_9: 0x1FF,
        }
    }

    pub fn update(&mut self) {
        if self.prev_index < 0.5 && self.index >= 0.5 {
            self.square ^= 0x1;
            self.poly_4 = ((self.poly_4 >> 1 & 0x1) ^ (self.poly_4 & 0x1)) << 3 | self.poly_4 >> 1;
            self.poly_5 = ((self.poly_5 >> 2 & 0x1) ^ (self.poly_5 & 0x1)) << 4 | self.poly_5 >> 1;
            self.poly_5_3 = ((self.poly_5_3 >> 2 & 0x1) ^ (self.poly_5_3 & 0x1)) << 4 | self.poly_5_3 >> 1;
            self.poly_9 = ((self.poly_9 >> 4 & 0x1) ^ (self.poly_9 & 0x1)) << 8 | self.poly_9 >> 1;

            if self.poly_5 & 0x1 == 1 { self.poly_4_3 = ((self.poly_4_3 >> 1 & 0x1) ^ (self.poly_4_3 & 0x1)) << 3 | self.poly_4_3 >> 1; }
        }
    }

    pub fn next_sample(&mut self) -> u8 {
        let frequency: f64 = NTSC_TIA_AUDIO_CLOCK / ((self.audf & 0x1F) + 1) as f64;
        let volume = 128 + (((self.audv & 0xF) as u16 * 127) / 15) as u8;
        let mut incr: f64 = 0.0;
        let mut sample = 0x80;

        match self.audc & 0xF {
            0x0 | 0xB => {
                incr = frequency / self.sample_rate as f64;
                sample = 0x80;
            }
            0x1 => {
                incr = frequency / self.sample_rate as f64;
                sample = if self.poly_4 & 0x1 == 0 {0x80} else {volume};
            }
            0x2 => {
                incr = (frequency / 15.0) / self.sample_rate as f64;
                sample = if self.poly_4 & 0x1 == 0 {0x80} else {volume};
            }
            0x3 => {
                incr = frequency / self.sample_rate as f64;
                sample = if self.poly_4_3 & 0x1 == 0 {0x80} else {volume};
            }
            0x4 | 0x5 => {
                incr = frequency / self.sample_rate as f64;
                sample = if self.square & 0x1 == 0 {0x80} else {volume};
            }
            0x6 | 0xA => {
                incr = (frequency / 15.5) / self.sample_rate as f64;
                sample = if self.square & 0x1 == 0 {0x80} else {volume};
            }
            0x7 | 0x9 => {
                incr = frequency / self.sample_rate as f64;
                sample = if self.poly_5 & 0x1 == 0 {0x80} else {volume};
            }
            0x8 => {
                incr = frequency / self.sample_rate as f64;
                sample = if self.poly_9 & 0x1 == 0 {0x80} else {volume};
            }
            0xC | 0xD => {
                incr = (frequency / 3.0) / self.sample_rate as f64;
                sample = if self.square & 0x1 == 0 {0x80} else {volume};
            }
            0xE => {
                incr = (frequency / 46.5) / self.sample_rate as f64;
                sample = if self.square & 0x1 == 0 {0x80} else {volume};
            }
            0xF => {
                // FIXME: Does not sound correct
                incr = (frequency / 6.0) / self.sample_rate as f64;
                sample = if self.poly_5 & 0x1 == 0 {0x80} else {volume};
            }
            _ => {}
        }
        self.prev_index = self.index;
        self.index += incr;
        if self.index > 1.0 { self.index -= 1.0 };

        self.update();

        sample
    }

    pub fn set_audc(&mut self, audio_control: u8) {
        self.audc = audio_control;
    }

    pub fn set_audf(&mut self, audio_frequency: u8) {
        self.audf = audio_frequency;
    }

    pub fn set_audv(&mut self, audio_volume: u8) {
        self.audv = audio_volume;
    }
}