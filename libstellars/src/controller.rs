pub struct Controller {
    swcha: u8,
    inpt: u8
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            swcha: 0b1111_1111,
            inpt: 0b1111_1111
        }
    }

    pub fn update_inputs(&mut self, mask: u8, pressed: bool, button: bool) {
        if button {
            if pressed {
                self.inpt &= !mask;
            } else {
                self.inpt |= mask;
            }
        } else if pressed {
            self.swcha &= !mask;
        } else {
            self.swcha |= mask;
        }
    }

    pub fn read_inputs(&self, address: u16) -> u8 {
        if address == 0x003D {
            self.inpt
        } else {
            self.swcha
        }
    }
}
