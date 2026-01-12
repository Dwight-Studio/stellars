#[repr(u16)]
#[derive(PartialEq)]
enum Functions {
    Inpt0 = 0x0008,
    Inpt1 = 0x0009,
    Inpt2 = 0x000A,
    Inpt3 = 0x000B,
    Inpt4 = 0x000C,
    Inpt5 = 0x000D,
    Swcha = 0x0280,
    Swacnt = 0x0281,
    Swchb = 0x0282,
    Swbcnt = 0x0283
}

#[derive(Copy, Clone)]
pub enum InputDevice {
    Joystick,
    Keypad
}

pub enum Input {
    Joystick(Joystick),
    Keypad(Keypad)
}

pub enum Keypad {
    R0C0,
    R0C1,
    R0C2,
    R1C0,
    R1C1,
    R1C2,
    R2C0,
    R2C1,
    R2C2,
    R3C0,
    R3C1,
    R3C2,
}

pub enum Joystick {
    Right,
    Left,
    Up,
    Down,
    Button
}

pub struct Controller {
    inpt0: u8,
    inpt1: u8,
    inpt2: u8,
    inpt3: u8,
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
            inpt0: 0b1111_1111,
            inpt1: 0b1111_1111,
            inpt2: 0b1111_1111,
            inpt3: 0b1111_1111,
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
                self.swacnt = 0x00;

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
            },
            Input::Keypad(keypad) => {
                self.swacnt = 0xFF;

                let (mask, address) = match keypad {
                    Keypad::R0C0 => (0b0001_0000, Functions::Inpt0),
                    Keypad::R0C1 => (0b0001_0000, Functions::Inpt1),
                    Keypad::R0C2 => (0b0001_0000, Functions::Inpt4),
                    Keypad::R1C0 => (0b0010_0000, Functions::Inpt0),
                    Keypad::R1C1 => (0b0010_0000, Functions::Inpt1),
                    Keypad::R1C2 => (0b0010_0000, Functions::Inpt4),
                    Keypad::R2C0 => (0b0100_0000, Functions::Inpt0),
                    Keypad::R2C1 => (0b0100_0000, Functions::Inpt1),
                    Keypad::R2C2 => (0b0100_0000, Functions::Inpt4),
                    Keypad::R3C0 => (0b1000_0000, Functions::Inpt0),
                    Keypad::R3C1 => (0b1000_0000, Functions::Inpt1),
                    Keypad::R3C2 => (0b1000_0000, Functions::Inpt4),
                };

                if pressed {
                    self.swcha &= !mask;
                } else {
                    self.swcha |= mask;
                }

                if address == Functions::Inpt0 {
                    if pressed {
                        self.inpt0 &= !0b1000_0000;
                    } else {
                        self.inpt0 |= 0b1000_0000;
                    }
                } else if address == Functions::Inpt1 {
                    if pressed {
                        self.inpt1 &= !0b1000_0000;
                    } else {
                        self.inpt1 |= 0b1000_0000;
                    }
                } else if address == Functions::Inpt4 {
                    if pressed {
                        self.inpt4 &= !0b1000_0000;
                    } else {
                        self.inpt4 |= 0b1000_0000;
                    }
                }
            }
        }
    }

    pub fn read_inputs(&self, mut address: u16) -> u8 {
        if !(0x0280..=0x0283).contains(&address) {
            address &= 0x000F;
        }

        if address == Functions::Inpt0 as u16 {
            return self.inpt0;
        } else if address == Functions::Inpt1 as u16 {
            return self.inpt1;
        } else if address == Functions::Inpt2 as u16 {
            return self.inpt2;
        } else if address == Functions::Inpt3 as u16 {
            return self.inpt3;
        } else if address == Functions::Inpt4 as u16 {
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
