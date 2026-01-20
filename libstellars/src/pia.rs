#[repr(u16)]
enum Functions {
    Intim = 0x00,
    Instat = 0x01,
    Tim1t = 0x14,
    Tim8t = 0x15,
    Tim64t = 0x16,
    T1024t = 0x17
}

#[derive(Copy, Clone)]
enum Timer {
    Tim1t,
    Tim8t,
    Tim64t,
    T1024t
}

#[derive(Copy, Clone)]
pub struct Pia {
    intim: u8,
    instat: u8,
    tim1t: u8,
    tim8t: u8,
    tim64t: u8,
    t1024t: u8,
    selected_timer: Timer,
    old_timer: Timer,
    internal_clock: i64
}

impl Pia {
    pub fn new() -> Self {
        Pia {
            intim: 0,
            instat: 0,
            tim1t: 0,
            tim8t: 0,
            tim64t: 0,
            t1024t: 0,
            selected_timer: Timer::Tim1t,
            old_timer: Timer::Tim1t,
            internal_clock: 1
        }
    }

    pub fn tick(&mut self) {
        self.internal_clock -= 1;

        if self.internal_clock == 0 {
            self.decrement();

            match self.selected_timer {
                Timer::Tim1t => self.internal_clock = 1,
                Timer::Tim8t => self.internal_clock = 8,
                Timer::Tim64t => self.internal_clock = 64,
                Timer::T1024t => self.internal_clock = 1024,
            }
        }
    }

    fn decrement(&mut self) {
        let (value, underflow) = self.intim.overflowing_sub(1);
        self.intim = value;

        if underflow {
            self.selected_timer = Timer::Tim1t;
            self.instat = 0b1100_0000;
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            x if x == Functions::Intim as u16 => {
                self.instat &= 0b1000_0000;
                self.selected_timer = self.old_timer;
                self.intim
            },
            x if x == Functions::Instat as u16 => self.instat,
            x if x == Functions::Tim1t as u16 => self.tim1t,
            x if x == Functions::Tim8t as u16 => self.tim8t,
            x if x == Functions::Tim64t as u16 => self.tim64t,
            x if x == Functions::T1024t as u16 => self.t1024t,
            _ => {0} // FIXME: Should not go here but it does in this rom: https://www.pouet.net/prod.php?which=57530
        }

    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.instat &= 0b0100_0000;

        match address {
            x if x == Functions::Tim1t as u16 => {
                self.tim1t = value;
                self.selected_timer = Timer::Tim1t;
                self.old_timer = Timer::Tim1t;
                self.internal_clock = 1;
            },
            x if x == Functions::Tim8t as u16 => {
                self.tim8t = value;
                self.selected_timer = Timer::Tim8t;
                self.old_timer = Timer::Tim8t;
                self.internal_clock = 8;
            },
            x if x == Functions::Tim64t as u16 => {
                self.tim64t = value;
                self.selected_timer = Timer::Tim64t;
                self.old_timer = Timer::Tim64t;
                self.internal_clock = 64;
            },
            x if x == Functions::T1024t as u16 => {
                self.t1024t = value;
                self.selected_timer = Timer::T1024t;
                self.old_timer = Timer::T1024t;
                self.internal_clock = 1024;
            },
            _ => unreachable!()
        }

        self.intim = value;

        self.decrement();
    }
}