#[repr(u16)]
enum Functions {
    Inpt4 = 0x003C,
    Swcha = 0x0280,
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
    swcha: u8,
    inpt4: u8,
}

impl Controller {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Controller {
            swcha: 0b1111_1111,
            inpt4: 0b1111_1111
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
            self.inpt4
        } else {
            self.swcha
        }
    }
}
