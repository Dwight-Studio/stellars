pub struct Counter {
    count: u8,
    max_count: u8,
}

impl Counter {
    pub fn new(max_count: u8) -> Self {
        Self {
            count: 0,
            max_count,
        }
    }

    pub fn update(&mut self) {
        self.count += 1;

        if self.count >= self.max_count { self.reset(); }
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    pub fn increment(&mut self, increment: u8) {
        for _ in 0..increment {
            self.update();
        }
    }

    pub fn decrement(&mut self, decrement: u8) {
        for _ in 0..decrement {
            if self.count == 0 {
                self.count = self.max_count;
                continue;
            }
            self.count -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }
}