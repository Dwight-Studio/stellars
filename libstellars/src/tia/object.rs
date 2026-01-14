use crate::SCREEN_WIDTH;
use crate::tia::register::Register;

pub(crate) struct Object {
    count: u8,
    can_draw: bool,
    vdel_new: u8,
    vdel_old: u8,

    should_move: bool,
    move_target: u8,
    curr_move: u8,
}

impl Object {
    pub fn new() -> Self {
        Self {
            count: 0,
            can_draw: true,
            vdel_new: 0x00,
            vdel_old: 0x00,

            should_move: false,
            move_target: 0,
            curr_move: 0,
        }
    }

    pub fn update(&mut self) {
        self.count += 1;
        self.can_draw = true;

        if self.count >= SCREEN_WIDTH as u8 { self.counter_reset(true); }

        if self.should_move {
            self.curr_move += 1;
            if self.curr_move == 15 * 4 { self.curr_move = 0; }
            if self.curr_move.is_multiple_of(4) {
                self.count += 1;
                if self.count >= SCREEN_WIDTH as u8 { self.counter_reset(true); }
            }
            if self.curr_move == self.move_target { self.should_move = false; self.curr_move = 0 }
        }
    }

    pub fn move_to(&mut self, target: u8) {
        self.should_move = true;
        self.move_target = target * 4;
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