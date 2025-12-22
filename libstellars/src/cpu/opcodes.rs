use crate::cpu::Cpu;

pub static OPCODES: [fn(&mut Cpu); 0x100] = {
    [|cpu| {
        /* 0x00 */
        /* BRK */
        cpu.registers.set_b(true);
        cpu.write_stack((cpu.registers.pc + 1) as u8);
        cpu.write_stack(cpu.registers.p);
        cpu.registers.set_i(true);
        cpu.registers.pc = cpu.read_byte(0xFFFE) as u16;
    },
    |cpu| {
        /* 0x01 */
        /* ORA (nn, X) */
        let nn = cpu.fetch_bytes();
        let lo_address = cpu.read_byte(nn.wrapping_add(cpu.registers.x) as u16);
        let hi_address = cpu.read_byte(nn.wrapping_add(cpu.registers.x).wrapping_add(1) as u16);
        let address = (hi_address as u16) << 8 | lo_address as u16;
        cpu.registers.acc |= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |_| {
        /* 0x02 */
    },
    |_| {
        /* 0x03 */
    },
    |_| {
        /* 0x04 */
    },
    |cpu| {
        /* 0x05 */
        /* ORA nn */
        let nn = cpu.fetch_bytes();
        let value = cpu.read_byte(nn as u16);
        cpu.registers.acc |= value;

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x06 */
        /* ASL nn */
        let nn = cpu.fetch_bytes();
        let old_value = cpu.read_byte(nn as u16);
        let result = old_value << 1;
        cpu.write_byte(nn as u16, result);

        cpu.registers.set_c(old_value >> 7 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(result >> 7 == 1);
    },
    |_| {
        /* 0x07 */
    },
    |cpu| {
        /* 0x08 */
        /* PHP */
        cpu.write_stack(cpu.registers.p);
    },
    |cpu| {
        /* 0x09 */
        /* ORA #nn */
        let nn = cpu.fetch_bytes();
        cpu.registers.acc |= nn;

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x0A */
        /* ASL A */
        let old_value = cpu.registers.acc;
        cpu.registers.acc <<= 1;

        cpu.registers.set_c(old_value >> 7 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |_| {
        /* 0x0B */
    },
    |_| {
        /* 0x0C */
    },
    |cpu| {
        /* 0x0D */
        /* ORA nnnn */
        let low_nn = cpu.fetch_bytes();
        let high_nn = cpu.fetch_bytes();
        let address = (high_nn as u16) << 8 | low_nn as u16;
        cpu.registers.acc |= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x0E */
        /* ASL nnnn */
        let low_nn = cpu.fetch_bytes();
        let high_nn = cpu.fetch_bytes();
        let address = (high_nn as u16) << 8 | low_nn as u16;
        let old_value = cpu.read_byte(address);
        let result = old_value << 1;
        cpu.write_byte(address, result);

        cpu.registers.set_c(old_value >> 7 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(result >> 7 == 1);
    },
    |_| {
        /* 0x0F */
    },
    |cpu| {
        /* 0x10 */
        /* BPL nnn */
        if !cpu.registers.get_n() {
            cpu.registers.pc = cpu.registers.pc.wrapping_add_signed(cpu.fetch_bytes() as i8 as i16);
        }
    },
    |cpu| {
        /* 0x11 */
        /* ORA (nn),Y */
        let nn = cpu.fetch_bytes();
        let lo_address = cpu.read_byte(nn as u16);
        let hi_address = cpu.read_byte((nn + 1) as u16);
        let address = (((hi_address as u16) << 8) | (lo_address as u16)) + cpu.registers.y as u16;
        cpu.registers.acc |= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |_| {
        /* 0x12 */
    },
    |_| {
        /* 0x13 */
    },
    |_| {
        /* 0x14 */
    },
    |cpu| {
        /* 0x15 */
        /* ORA nn,X */
        let nn = cpu.fetch_bytes();
        let address = nn.wrapping_add(cpu.registers.x);
        cpu.registers.acc |= cpu.read_byte(address as u16);
    },
    |_| {
        /* 0x16 */
    },
    |_| {
        /* 0x17 */
    },
    |_| {
        /* 0x18 */
    },
    |_| {
        /* 0x19 */
    },
    |_| {
        /* 0x1A */
    },
    |_| {
        /* 0x1B */
    },
    |_| {
        /* 0x1C */
    },
    |_| {
        /* 0x1D */
    },
    |_| {
        /* 0x1E */
    },
    |_| {
        /* 0x1F */
    },
    |_| {
        /* 0x20 */
    },
    |_| {
        /* 0x21 */
    },
    |_| {
        /* 0x22 */
    },
    |_| {
        /* 0x23 */
    },
    |_| {
        /* 0x24 */
    },
    |_| {
        /* 0x25 */
    },
    |_| {
        /* 0x26 */
    },
    |_| {
        /* 0x27 */
    },
    |_| {
        /* 0x28 */
    },
    |_| {
        /* 0x29 */
    },
    |_| {
        /* 0x2A */
    },
    |_| {
        /* 0x2B */
    },
    |_| {
        /* 0x2C */
    },
    |_| {
        /* 0x2D */
    },
    |_| {
        /* 0x2E */
    },
    |_| {
        /* 0x2F */
    },
    |_| {
        /* 0x30 */
    },
    |_| {
        /* 0x31 */
    },
    |_| {
        /* 0x32 */
    },
    |_| {
        /* 0x33 */
    },
    |_| {
        /* 0x34 */
    },
    |_| {
        /* 0x35 */
    },
    |_| {
        /* 0x36 */
    },
    |_| {
        /* 0x37 */
    },
    |_| {
        /* 0x38 */
    },
    |_| {
        /* 0x39 */
    },
    |_| {
        /* 0x3A */
    },
    |_| {
        /* 0x3B */
    },
    |_| {
        /* 0x3C */
    },
    |_| {
        /* 0x3D */
    },
    |_| {
        /* 0x3E */
    },
    |_| {
        /* 0x3F */
    },
    |_| {
        /* 0x40 */
    },
    |_| {
        /* 0x41 */
    },
    |_| {
        /* 0x42 */
    },
    |_| {
        /* 0x43 */
    },
    |_| {
        /* 0x44 */
    },
    |_| {
        /* 0x45 */
    },
    |_| {
        /* 0x46 */
    },
    |_| {
        /* 0x47 */
    },
    |_| {
        /* 0x48 */
    },
    |_| {
        /* 0x49 */
    },
    |_| {
        /* 0x4A */
    },
    |_| {
        /* 0x4B */
    },
    |_| {
        /* 0x4C */
    },
    |_| {
        /* 0x4D */
    },
    |_| {
        /* 0x4E */
    },
    |_| {
        /* 0x4F */
    },
    |_| {
        /* 0x50 */
    },
    |_| {
        /* 0x51 */
    },
    |_| {
        /* 0x52 */
    },
    |_| {
        /* 0x53 */
    },
    |_| {
        /* 0x54 */
    },
    |_| {
        /* 0x55 */
    },
    |_| {
        /* 0x56 */
    },
    |_| {
        /* 0x57 */
    },
    |_| {
        /* 0x58 */
    },
    |_| {
        /* 0x59 */
    },
    |_| {
        /* 0x5A */
    },
    |_| {
        /* 0x5B */
    },
    |_| {
        /* 0x5C */
    },
    |_| {
        /* 0x5D */
    },
    |_| {
        /* 0x5E */
    },
    |_| {
        /* 0x5F */
    },
    |_| {
        /* 0x60 */
    },
    |_| {
        /* 0x61 */
    },
    |_| {
        /* 0x62 */
    },
    |_| {
        /* 0x63 */
    },
    |_| {
        /* 0x64 */
    },
    |_| {
        /* 0x65 */
    },
    |_| {
        /* 0x66 */
    },
    |_| {
        /* 0x67 */
    },
    |_| {
        /* 0x68 */
    },
    |_| {
        /* 0x69 */
    },
    |_| {
        /* 0x6A */
    },
    |_| {
        /* 0x6B */
    },
    |_| {
        /* 0x6C */
    },
    |_| {
        /* 0x6D */
    },
    |_| {
        /* 0x6E */
    },
    |_| {
        /* 0x6F */
    },
    |_| {
        /* 0x70 */
    },
    |_| {
        /* 0x71 */
    },
    |_| {
        /* 0x72 */
    },
    |_| {
        /* 0x73 */
    },
    |_| {
        /* 0x74 */
    },
    |_| {
        /* 0x75 */
    },
    |_| {
        /* 0x76 */
    },
    |_| {
        /* 0x77 */
    },
    |_| {
        /* 0x78 */
    },
    |_| {
        /* 0x79 */
    },
    |_| {
        /* 0x7A */
    },
    |_| {
        /* 0x7B */
    },
    |_| {
        /* 0x7C */
    },
    |_| {
        /* 0x7D */
    },
    |_| {
        /* 0x7E */
    },
    |_| {
        /* 0x7F */
    },
    |_| {
        /* 0x80 */
    },
    |_| {
        /* 0x81 */
    },
    |_| {
        /* 0x82 */
    },
    |_| {
        /* 0x83 */
    },
    |_| {
        /* 0x84 */
    },
    |_| {
        /* 0x85 */
    },
    |_| {
        /* 0x86 */
    },
    |_| {
        /* 0x87 */
    },
    |_| {
        /* 0x88 */
    },
    |_| {
        /* 0x89 */
    },
    |_| {
        /* 0x8A */
    },
    |_| {
        /* 0x8B */
    },
    |_| {
        /* 0x8C */
    },
    |_| {
        /* 0x8D */
    },
    |_| {
        /* 0x8E */
    },
    |_| {
        /* 0x8F */
    },
    |_| {
        /* 0x90 */
    },
    |_| {
        /* 0x91 */
    },
    |_| {
        /* 0x92 */
    },
    |_| {
        /* 0x93 */
    },
    |_| {
        /* 0x94 */
    },
    |_| {
        /* 0x95 */
    },
    |_| {
        /* 0x96 */
    },
    |_| {
        /* 0x97 */
    },
    |_| {
        /* 0x98 */
    },
    |_| {
        /* 0x99 */
    },
    |_| {
        /* 0x9A */
    },
    |_| {
        /* 0x9B */
    },
    |_| {
        /* 0x9C */
    },
    |_| {
        /* 0x9D */
    },
    |_| {
        /* 0x9E */
    },
    |_| {
        /* 0x9F */
    },
    |_| {
        /* 0xA0 */
    },
    |_| {
        /* 0xA1 */
    },
    |_| {
        /* 0xA2 */
    },
    |_| {
        /* 0xA3 */
    },
    |_| {
        /* 0xA4 */
    },
    |_| {
        /* 0xA5 */
    },
    |_| {
        /* 0xA6 */
    },
    |_| {
        /* 0xA7 */
    },
    |_| {
        /* 0xA8 */
    },
    |_| {
        /* 0xA9 */
    },
    |_| {
        /* 0xAA */
    },
    |_| {
        /* 0xAB */
    },
    |_| {
        /* 0xAC */
    },
    |_| {
        /* 0xAD */
    },
    |_| {
        /* 0xAE */
    },
    |_| {
        /* 0xAF */
    },
    |_| {
        /* 0xB0 */
    },
    |_| {
        /* 0xB1 */
    },
    |_| {
        /* 0xB2 */
    },
    |_| {
        /* 0xB3 */
    },
    |_| {
        /* 0xB4 */
    },
    |_| {
        /* 0xB5 */
    },
    |_| {
        /* 0xB6 */
    },
    |_| {
        /* 0xB7 */
    },
    |_| {
        /* 0xB8 */
    },
    |_| {
        /* 0xB9 */
    },
    |_| {
        /* 0xBA */
    },
    |_| {
        /* 0xBB */
    },
    |_| {
        /* 0xBC */
    },
    |_| {
        /* 0xBD */
    },
    |_| {
        /* 0xBE */
    },
    |_| {
        /* 0xBF */
    },
    |_| {
        /* 0xC0 */
    },
    |_| {
        /* 0xC1 */
    },
    |_| {
        /* 0xC2 */
    },
    |_| {
        /* 0xC3 */
    },
    |_| {
        /* 0xC4 */
    },
    |_| {
        /* 0xC5 */
    },
    |_| {
        /* 0xC6 */
    },
    |_| {
        /* 0xC7 */
    },
    |_| {
        /* 0xC8 */
    },
    |_| {
        /* 0xC9 */
    },
    |_| {
        /* 0xCA */
    },
    |_| {
        /* 0xCB */
    },
    |_| {
        /* 0xCC */
    },
    |_| {
        /* 0xCD */
    },
    |_| {
        /* 0xCE */
    },
    |_| {
        /* 0xCF */
    },
    |_| {
        /* 0xD0 */
    },
    |_| {
        /* 0xD1 */
    },
    |_| {
        /* 0xD2 */
    },
    |_| {
        /* 0xD3 */
    },
    |_| {
        /* 0xD4 */
    },
    |_| {
        /* 0xD5 */
    },
    |_| {
        /* 0xD6 */
    },
    |_| {
        /* 0xD7 */
    },
    |_| {
        /* 0xD8 */
    },
    |_| {
        /* 0xD9 */
    },
    |_| {
        /* 0xDA */
    },
    |_| {
        /* 0xDB */
    },
    |_| {
        /* 0xDC */
    },
    |_| {
        /* 0xDD */
    },
    |_| {
        /* 0xDE */
    },
    |_| {
        /* 0xDF */
    },
    |_| {
        /* 0xE0 */
    },
    |_| {
        /* 0xE1 */
    },
    |_| {
        /* 0xE2 */
    },
    |_| {
        /* 0xE3 */
    },
    |_| {
        /* 0xE4 */
    },
    |_| {
        /* 0xE5 */
    },
    |_| {
        /* 0xE6 */
    },
    |_| {
        /* 0xE7 */
    },
    |_| {
        /* 0xE8 */
    },
    |_| {
        /* 0xE9 */
    },
    |_| {
        /* 0xEA */
    },
    |_| {
        /* 0xEB */
    },
    |_| {
        /* 0xEC */
    },
    |_| {
        /* 0xED */
    },
    |_| {
        /* 0xEE */
    },
    |_| {
        /* 0xEF */
    },
    |_| {
        /* 0xF0 */
    },
    |_| {
        /* 0xF1 */
    },
    |_| {
        /* 0xF2 */
    },
    |_| {
        /* 0xF3 */
    },
    |_| {
        /* 0xF4 */
    },
    |_| {
        /* 0xF5 */
    },
    |_| {
        /* 0xF6 */
    },
    |_| {
        /* 0xF7 */
    },
    |_| {
        /* 0xF8 */
    },
    |_| {
        /* 0xF9 */
    },
    |_| {
        /* 0xFA */
    },
    |_| {
        /* 0xFB */
    },
    |_| {
        /* 0xFC */
    },
    |_| {
        /* 0xFD */
    },
    |_| {
        /* 0xFE */
    },
    |_| {
        /* 0xFF */
    }]
};