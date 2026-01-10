use crate::SCREEN_WIDTH;

pub(crate) struct Object {
    count: u8,
    can_draw: bool,
}

impl Object {
    pub fn new() -> Self {
        Self {
            count: 0,
            can_draw: false,
        }
    }

    pub fn update(&mut self) {
        self.count += 1;
        self.can_draw = true;

        if self.count >= SCREEN_WIDTH as u8 { self.counter_reset(true); }
    }

    pub fn count(&self) -> u8 {
        self.count
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

    pub fn can_draw(&self) -> bool {
        self.can_draw
    }
}