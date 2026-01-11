#[repr(u16)]
enum Functions {
    Inpt4 = 0x000C,
    Inpt5 = 0x000D,
    Swcha = 0x0280,
    Swacnt = 0x0281,
    Swchb = 0x0282,
    Swbcnt = 0x0283
}

pub enum Input {
    Joystick(Joystick)
}

pub enum Joystick {
    Right,
    Left,
    Up,
    Down,
    Button
}

pub struct Controller {
    inpt4: u8,
    inpt5: u8,
    swcha: u8,
    swacnt: u8,
    swchb: u8,
    swbcnt: u8
}

impl Controller {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Controller {
            inpt4: 0b1111_1111,
            inpt5: 0b1111_1111,
            swcha: 0b1111_1111,
            swacnt: 0x00,
            swchb: 0b0000_1011,
            swbcnt: 0x00
        }
    }


    pub fn update_inputs(&mut self, input: Input, pressed: bool) {
        match input {
            Input::Joystick(joystick) => {
                let (mask, button) = match joystick {
                    Joystick::Right => (0b1000_0000, false),
                    Joystick::Left => (0b0100_0000, false),
                    Joystick::Up => (0b0001_0000, false),
                    Joystick::Down => (0b0010_0000, false),
                    Joystick::Button => (0b1000_0000, true),
                };

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
        }
    }

    pub fn read_inputs(&self, address: u16) -> u8 {
        if address == Functions::Inpt4 as u16 {
            return self.inpt4;
        } else if address == Functions::Inpt5 as u16 {
            return self.inpt5;
        } else if address == Functions::Swcha as u16 {
            return self.swcha;
        } else if address == Functions::Swacnt as u16 {
            return self.swacnt;
        } else if address == Functions::Swchb as u16 {
            return self.swchb;
        } else if address == Functions::Swbcnt as u16 {
            return self.swbcnt;
        }

        0xFF
    }
}
