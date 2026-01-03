#[repr(u16)]
enum Functions {
    Inpt4 = 0x003C,
    Swcha = 0x0280,
}

pub struct Controller {
    swcha: u8,
    inpt4: u8,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            swcha: 0b1111_1111,
            inpt4: 0b1111_1111
        }
    }

    pub fn update_inputs(&mut self, mask: u8, pressed: bool, button: bool) {
        if button {
            if pressed {
                self.inpt4 &= !mask;
            } else {
                self.inpt4 |= mask;
            }
        } else if pressed {
            self.swcha &= !mask;
        } else {
            self.swcha |= mask;
        }
    }

    pub fn read_inputs(&self, address: u16) -> u8 {
        if address == Functions::Inpt4 as u16 {
            self.inpt4
        } else {
            self.swcha
        }
    }
}
