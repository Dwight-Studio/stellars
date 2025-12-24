use std::cell::RefCell;
use std::rc::Rc;
use crate::Stellar;

#[repr(u8)]
pub enum WriteFunctions {
    VSYNC,
    VBLANK,
    WSYNC,
    RSYNC,
    NUSIZ0,
    NUSIZ1,
    COLUP0,
    COLUP1,
    COLUPF,
    COLUBK,
    CTRLPF,
    REFP0,
    REFP1,
    PF0,
    PF1,
    PF2,
    RESP0,
    RESP1,
    RESM0,
    RESM1,
    RESBL,
    AUDC0,
    AUDC1,
    AUDF0,
    AUDF1,
    AUDV0,
    AUDV1,
    GRP0,
    GRP1,
    ENAM0,
    ENAM1,
    ENABL,
    HMP0,
    HMP1,
    HMM0,
    HMM1,
    HMBL,
    VDELP0,
    VDELP1,
    VDELBL,
    RESMP0,
    RESMP1,
    HMOVE,
    HMCLR,
    CXCLR
}
#[repr(u8)]
pub enum ReadFunctions {
    CXM0P,
    CXM1P,
    CXP0FB,
    CXP1FB,
    CXM0FB,
    CXM1FB,
    CXPPMM,
    INPT0,
    INPT1,
    INPT2,
    INPT3,
    INPT4,
    INPT5,
}

pub struct Tia {
    pub(crate) bus: Option<Rc<RefCell<Stellar>>>,
    write_functions: [u8; 0x2D],
    read_functions: [u8; 0x0E],

    pic_x: u8,
    pic_y: u8,
}

impl Tia {
    pub fn new(bus: Option<Rc<RefCell<Stellar>>>) -> Tia {
        Self {
            bus,
            write_functions: [0x00; 0x2D],
            read_functions: [0; 0x0E],

            pic_x: 0x0000,
            pic_y: 0x0000,
        }
    }

    pub fn set_write_function(&mut self, address: u8, value: u8) {
        self.write_functions[address as usize] = value;
    }

    pub fn get_write_function(&self, address: WriteFunctions) -> u8 {
        self.write_functions[address as usize]
    }

    pub fn tick(&mut self, cycles: u64) {
        for _ in 0..cycles * 3 {
            if (self.get_write_function(WriteFunctions::VSYNC) >> 1) & 0x1 == 0x1 ||
                (self.get_write_function(WriteFunctions::VBLANK) >> 1) & 0x1 == 0x1 {
                self.pic_x = 0x00;
                self.pic_y = 0x00;
                break;
            }

            if self.pic_x >= 228 {
                self.pic_x = 0;
                self.pic_y += 1;
            }

            if self.pic_y >= 192 {
                break;
            }

            if self.pic_x >= 68 {
                todo!("Draw the picture")
            }

            self.pic_x += 1;
        }
    }

    fn get_color_from_register(&self, write_register: WriteFunctions) -> (u8, u8, u8) {
        unimplemented!()
    }
}