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
    
    pub fn four_bits_twos_complement(&self) -> i8 {
        let reg = self.value >> 4 & 0x7;
        
        if self.bit(3) {
            reg as i8
        } else {
            reg as i8 - 8
        }
    } 
}