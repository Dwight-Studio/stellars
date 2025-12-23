pub struct Registers {
    pub(crate) acc: u8,
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) pc: u16,
    pub(crate) sp: u8,
    pub(crate) p: u8
}

impl Registers {
    //fixme: pc doit être égal à [0xFFFC] sauf que la mémoire n'existe pas encore ici
    pub fn new() -> Self {
        Registers {
            acc: 0x00,
            x: 0x00,
            y: 0x00,
            pc: 0xFFFC,
            sp: 0xFF,
            p: 0b0010_0110
        }
    }

    pub fn get_n(&self) -> bool {
        self.p & 0b1000_0000 != 0
    }

    pub fn get_v(&self) -> bool {
        self.p & 0b0100_0000 != 0
    }

    pub fn get_b(&self) -> bool {
        self.p & 0b0001_0000 != 0
    }

    pub fn get_d(&self) -> bool {
        self.p & 0b0000_1000 != 0
    }

    pub fn get_i(&self) -> bool {
        self.p & 0b0000_0100 != 0
    }

    pub fn get_z(&self) -> bool {
        self.p & 0b0000_0010 != 0
    }

    pub fn get_c(&self) -> bool {
        self.p & 0b0000_0001 != 0
    }

    pub fn set_n(&mut self, toggle: bool) {
        if toggle {
            self.p |= 0b1000_0000;
        } else {
            self.p &= !0b1000_0000;
        }
    }

    pub fn set_v(&mut self, toggle: bool) {
        if toggle {
            self.p |= 0b0100_0000;
        } else {
            self.p &= !0b0100_0000;
        }
    }

    pub fn set_b(&mut self, toggle: bool) {
        if toggle {
            self.p |= 0b0001_0000;
        } else {
            self.p &= !0b0001_0000;
        }
    }

    pub fn set_d(&mut self, toggle: bool) {
        if toggle {
            self.p |= 0b0000_1000;
        } else {
            self.p &= !0b0000_1000;
        }
    }

    pub fn set_i(&mut self, toggle: bool) {
        if toggle {
            self.p |= 0b0000_0100;
        } else {
            self.p &= !0b0000_0100;
        }
    }

    pub fn set_z(&mut self, toggle: bool) {
        if toggle {
            self.p |= 0b0000_0010;
        } else {
            self.p &= !0b0000_0010;
        }
    }

    pub fn set_c(&mut self, toggle: bool) {
        if toggle {
            self.p |= 0b0000_0001;
        } else {
            self.p &= !0b0000_0001;
        }
    }
}