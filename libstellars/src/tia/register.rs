pub struct Register {
    pub value: u8
}

impl Register {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn bit(&self, n: u8) -> bool {
        (self.value >> n) & 0x1 == 1
    }
}