use crate::tia::register::Register;

pub(crate) struct Object {
    count: u8,
    can_draw: bool,
    vdel_new: u8,
    vdel_old: u8,
}

impl Object {
    pub fn new() -> Self {
        Self {
            count: 0,
            can_draw: true,
            vdel_new: 0x00,
            vdel_old: 0x00,
        }
    }

    pub fn update(&mut self) {
        self.count += 1;
        self.can_draw = true;

        if self.count >= 160 { self.counter_reset(true); }
    }

    pub fn counter_increment(&mut self, increment: u8) {
        for _ in 0..increment {
            self.update();
        }
    }

    pub fn counter_reset(&mut self, can_draw: bool) {
        self.count = 0;
        self.can_draw = can_draw;
    }
    
    pub fn set_vdel_new(&mut self, new: u8) {
        self.vdel_new = new;
    }
    
    pub fn get_vdel_new(&self) -> Register {
        Register::new(self.vdel_new)
    }

    pub fn set_vdel_old(&mut self, old: u8) {
        self.vdel_old = old;
    }
    
    pub fn get_vdel_old(&self) -> Register {
        Register::new(self.vdel_old)
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    pub fn can_draw(&self) -> bool {
        self.can_draw
    }
}