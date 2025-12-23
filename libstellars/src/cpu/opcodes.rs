use crate::cpu::Cpu;

pub static OPCODES: [fn(&mut Cpu); 0x100] = {
    [|cpu| {
        /* 0x00 */
        /* BRK */
        cpu.registers.set_b(true);
        cpu.push_stack((cpu.registers.pc + 1) as u8);
        cpu.push_stack(cpu.registers.p);
        cpu.registers.set_i(true);
        cpu.registers.pc = cpu.read_byte(0xFFFE) as u16;
    },
    |cpu| {
        /* 0x01 */
        /* ORA (nn, X) */
        let nn = cpu.fetch_byte();
        let low_address = cpu.read_byte(nn.wrapping_add(cpu.registers.x) as u16);
        let high_address = cpu.read_byte(nn.wrapping_add(cpu.registers.x).wrapping_add(1) as u16);
        let address = (high_address as u16) << 8 | low_address as u16;
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
        let nn = cpu.fetch_byte();
        let value = cpu.read_byte(nn as u16);
        cpu.registers.acc |= value;

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x06 */
        /* ASL nn */
        let nn = cpu.fetch_byte();
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
        cpu.push_stack(cpu.registers.p);
    },
    |cpu| {
        /* 0x09 */
        /* ORA #nn */
        let nn = cpu.fetch_byte();
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
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = (high_nn as u16) << 8 | low_nn as u16;
        cpu.registers.acc |= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x0E */
        /* ASL nnnn */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
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
            cpu.registers.pc = cpu.registers.pc.wrapping_add_signed(cpu.fetch_byte() as i8 as i16);
        }
    },
    |cpu| {
        /* 0x11 */
        /* ORA (nn),Y */
        let nn = cpu.fetch_byte();
        let low_address = cpu.read_byte(nn as u16);
        let high_address = cpu.read_byte((nn + 1) as u16);
        let address = ((high_address as u16) << 8 | low_address as u16) + cpu.registers.y as u16;
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
        let nn = cpu.fetch_byte();
        let address = nn.wrapping_add(cpu.registers.x);
        cpu.registers.acc |= cpu.read_byte(address as u16);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x16 */
        /* ASL nn,X */
        let nn = cpu.fetch_byte();
        let address = nn.wrapping_add(cpu.registers.x);
        let old_value = cpu.read_byte(address as u16);
        let result = old_value << 1;
        cpu.write_byte(address as u16, result);

        cpu.registers.set_c(old_value >> 7 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(result >> 7 == 1);
    },
    |_| {
        /* 0x17 */
    },
    |cpu| {
        /* 0x18 */
        /* CLC */
        cpu.registers.set_c(false);
    },
    |cpu| {
        /* 0x19 */
        /* ORA nnnn,Y */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = ((high_nn as u16) << 8 | low_nn as u16) + cpu.registers.y as u16;
        cpu.registers.acc |= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
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
    |cpu| {
        /* 0x1D */
        /* ORA nnnn,X */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = ((high_nn as u16) << 8 | low_nn as u16) + cpu.registers.x as u16;
        cpu.registers.acc |= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x1E */
        /* ASL nnnn,X */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = ((high_nn as u16) << 8 | low_nn as u16) + cpu.registers.x as u16;
        let old_value = cpu.read_byte(address);
        let result = old_value << 1;
        cpu.write_byte(address, result);

        cpu.registers.set_c(old_value >> 7 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(result >> 7 == 1);
    },
    |_| {
        /* 0x1F */
    },
    |cpu| {
        /* 0x20 */
        /* JSR nnnn */
        cpu.push_stack(((cpu.registers.pc + 1) >> 8) as u8);
        cpu.push_stack((cpu.registers.pc + 1) as u8);
        let low_address = cpu.fetch_byte();
        let high_address = cpu.fetch_byte();
        cpu.registers.pc = (high_address as u16) << 8 | low_address as u16;
    },
    |cpu| {
        /* 0x21 */
        /* AND (nn,X) */
        let nn = cpu.fetch_byte();
        let low_address = cpu.read_byte(nn.wrapping_add(cpu.registers.x) as u16);
        let high_address = cpu.read_byte(nn.wrapping_add(cpu.registers.x).wrapping_add(1) as u16);
        let address = (high_address as u16) << 8 | low_address as u16;
        cpu.registers.acc &= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |_| {
        /* 0x22 */
    },
    |_| {
        /* 0x23 */
    },
    |cpu| {
        /* 0x24 */
        /* BIT nn */
        let nn = cpu.fetch_byte();
        let value = cpu.read_byte(nn as u16);
        let result = cpu.registers.acc & value;

        cpu.registers.set_z(result == 0);
        cpu.registers.set_n(value >> 7 == 1);
        cpu.registers.set_v(value & 0b0100_0000 != 0);
    },
    |cpu| {
        /* 0x25 */
        /* AND nn */
        let nn = cpu.fetch_byte();
        let value = cpu.read_byte(nn as u16);
        cpu.registers.acc &= value;

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x26 */
        /* ROL nn */
        let nn = cpu.fetch_byte();
        let old_value = cpu.read_byte(nn as u16);
        let low_value = cpu.registers.get_c() as u8;
        let result = (old_value << 1) | low_value;
        cpu.write_byte(nn as u16, result);

        cpu.registers.set_c(old_value >> 7 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(result >> 7 == 1);
    },
    |_| {
        /* 0x27 */
    },
    |cpu| {
        /* 0x28 */
        /* PLP */
        cpu.registers.p = cpu.pull_stack();
    },
    |cpu| {
        /* 0x29 */
        /* AND #nn */
        let nn = cpu.fetch_byte();
        cpu.registers.acc &= nn;

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x2A */
        /* ROL A */
        let old_value = cpu.registers.acc;
        let low_value = cpu.registers.get_c() as u8;
        cpu.registers.acc = (old_value << 1) | low_value;

        cpu.registers.set_c(old_value >> 7 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |_| {
        /* 0x2B */
    },
    |cpu| {
        /* 0x2C */
        /* BIT nnnn */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = (high_nn as u16) << 8 | low_nn as u16;
        let value = cpu.read_byte(address);
        let result = cpu.registers.acc & value;

        cpu.registers.set_z(result == 0);
        cpu.registers.set_n(value >> 7 == 1);
        cpu.registers.set_v(value & 0b0100_0000 != 0);
    },
    |cpu| {
        /* 0x2D */
        /* AND nnnn */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = (high_nn as u16) << 8 | low_nn as u16;
        cpu.registers.acc &= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x2E */
        /* ROL nnnn */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = (high_nn as u16) << 8 | low_nn as u16;
        let old_value = cpu.read_byte(address);
        let low_value = cpu.registers.get_c() as u8;
        let result = (old_value << 1) | low_value;
        cpu.write_byte(address, result);

        cpu.registers.set_c(old_value >> 7 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(result >> 7 == 1);
    },
    |_| {
        /* 0x2F */
    },
    |cpu| {
        /* 0x30 */
        /* BMI nn */
        if cpu.registers.get_n() {
            cpu.registers.pc = cpu.registers.pc.wrapping_add_signed(cpu.fetch_byte() as i8 as i16);
        }
    },
    |cpu| {
        /* 0x31 */
        /* AND (nn),Y */
        let nn = cpu.fetch_byte();
        let low_address = cpu.read_byte(nn as u16);
        let high_address = cpu.read_byte((nn + 1) as u16);
        let address = ((high_address as u16) << 8 | low_address as u16) + cpu.registers.y as u16;
        cpu.registers.acc &= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
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
    |cpu| {
        /* 0x35 */
        /* AND nn,X */
        let nn = cpu.fetch_byte();
        let address = nn.wrapping_add(cpu.registers.x);
        cpu.registers.acc &= cpu.read_byte(address as u16);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x36 */
        /* ROL nn,X */
        let nn = cpu.fetch_byte();
        let address = nn.wrapping_add(cpu.registers.x);
        let old_value = cpu.read_byte(address as u16);
        let low_value = cpu.registers.get_c() as u8;
        let result = (old_value << 1) | low_value;
        cpu.write_byte(address as u16, result);

        cpu.registers.set_c(old_value >> 7 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(result >> 7 == 1);
    },
    |_| {
        /* 0x37 */
    },
    |cpu| {
        /* 0x38 */
        /* SEC */
        cpu.registers.set_c(true);
    },
    |cpu| {
        /* 0x39 */
        /* AND nnnn,Y */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = ((high_nn as u16) << 8 | low_nn as u16) + cpu.registers.y as u16;
        cpu.registers.acc &= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
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
    |cpu| {
        /* 0x3D */
        /* AND nnnn,X */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = ((high_nn as u16) << 8 | low_nn as u16) + cpu.registers.x as u16;
        cpu.registers.acc &= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x3E */
        /* ROL nnnn,X */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = ((high_nn as u16) << 8 | low_nn as u16) + cpu.registers.x as u16;
        let old_value = cpu.read_byte(address);
        let low_value = cpu.registers.get_c() as u8;
        let result = (old_value << 1) | low_value;
        cpu.write_byte(address, result);

        cpu.registers.set_c(old_value >> 7 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(result >> 7 == 1);
    },
    |_| {
        /* 0x3F */
    },
    |cpu| {
        /* 0x40 */
        /* RTI */
        cpu.registers.p = cpu.pull_stack();
        cpu.registers.pc = cpu.pull_stack() as u16;
    },
    |cpu| {
        /* 0x41 */
        /* EOR (nn,X) */
        let nn = cpu.fetch_byte();
        let low_address = cpu.read_byte(nn.wrapping_add(cpu.registers.x) as u16);
        let high_address = cpu.read_byte(nn.wrapping_add(cpu.registers.x).wrapping_add(1) as u16);
        let address = (high_address as u16) << 8 | low_address as u16;
        cpu.registers.acc ^= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
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
    |cpu| {
        /* 0x45 */
        /* EOR nn */
        let nn = cpu.fetch_byte();
        let value = cpu.read_byte(nn as u16);
        cpu.registers.acc ^= value;

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x46 */
        /* LSR nn */
        let nn = cpu.fetch_byte();
        let old_value = cpu.read_byte(nn as u16);
        let result = old_value >> 1;
        cpu.write_byte(nn as u16, result);

        cpu.registers.set_c(old_value & 0b0000_0001 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(false);
    },
    |_| {
        /* 0x47 */
    },
    |cpu| {
        /* 0x48 */
        /* PHA */
        cpu.push_stack(cpu.registers.acc);
    },
    |cpu| {
        /* 0x49 */
        /* EOR #nn */
        let nn = cpu.fetch_byte();
        cpu.registers.acc ^= nn;

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x4A */
        /* LSR A */
        let old_value = cpu.registers.acc;
        cpu.registers.acc >>= 1;

        cpu.registers.set_c(old_value & 0b0000_0001 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(false);
    },
    |_| {
        /* 0x4B */
    },
    |cpu| {
        /* 0x4C */
        /* JMP nnnn */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = (high_nn as u16) << 8 | low_nn as u16;
        cpu.registers.pc = address;
    },
    |cpu| {
        /* 0x4D */
        /* EOR nnnn */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = (high_nn as u16) << 8 | low_nn as u16;
        cpu.registers.acc ^= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x4E */
        /* LSR nnnn */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = (high_nn as u16) << 8 | low_nn as u16;
        let old_value = cpu.read_byte(address);
        let result = old_value >> 1;
        cpu.write_byte(address, result);

        cpu.registers.set_c(old_value & 0b0000_0001 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(false);
    },
    |_| {
        /* 0x4F */
    },
    |cpu| {
        /* 0x50 */
        /* BVC nn */
        if !cpu.registers.get_v() {
            cpu.registers.pc = cpu.registers.pc.wrapping_add_signed(cpu.fetch_byte() as i8 as i16);
        }
    },
    |cpu| {
        /* 0x51 */
        /* EOR (nn),Y */
        let nn = cpu.fetch_byte();
        let low_address = cpu.read_byte(nn as u16);
        let high_address = cpu.read_byte((nn + 1) as u16);
        let address = ((high_address as u16) << 8 | low_address as u16) + cpu.registers.y as u16;
        cpu.registers.acc ^= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
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
    |cpu| {
        /* 0x55 */
        /* EOR nn,X */
        let nn = cpu.fetch_byte();
        let address = nn.wrapping_add(cpu.registers.x);
        cpu.registers.acc ^= cpu.read_byte(address as u16);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x56 */
        /* LSR nn,X */
        let nn = cpu.fetch_byte();
        let address = nn.wrapping_add(cpu.registers.x);
        let old_value = cpu.read_byte(address as u16);
        let result = old_value >> 1;
        cpu.write_byte(address as u16, result);

        cpu.registers.set_c(old_value & 0b0000_0001 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(false);
    },
    |_| {
        /* 0x57 */
    },
    |cpu| {
        /* 0x58 */
        /* CLI */
        cpu.registers.set_i(false);
    },
    |cpu| {
        /* 0x59 */
        /* EOR nnnn,Y */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = ((high_nn as u16) << 8 | low_nn as u16) + cpu.registers.y as u16;
        cpu.registers.acc ^= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
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
    |cpu| {
        /* 0x5D */
        /* EOR nnnn,X */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = ((high_nn as u16) << 8 | low_nn as u16) + cpu.registers.x as u16;
        cpu.registers.acc ^= cpu.read_byte(address);

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x5E */
        /* LSR nnnn,X */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = ((high_nn as u16) << 8 | low_nn as u16) + cpu.registers.x as u16;
        let old_value = cpu.read_byte(address);
        let result = old_value >> 1;
        cpu.write_byte(address, result);

        cpu.registers.set_c(old_value & 0b0000_0001 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(false);
    },
    |_| {
        /* 0x5F */
    },
    |cpu| {
        /* 0x60 */
        /* RTS */
        let low_address = cpu.pull_stack();
        let high_address = cpu.pull_stack();
        let new_pc = (high_address as u16) << 8 | low_address as u16;
        cpu.registers.pc = new_pc + 1;
    },
    |cpu| {
        /* 0x61 */
        /* ADC (nn,X) */
        let nn = cpu.fetch_byte();
        let low_address = cpu.read_byte(nn.wrapping_add(cpu.registers.x) as u16);
        let high_address = cpu.read_byte(nn.wrapping_add(cpu.registers.x).wrapping_add(1) as u16);
        let address = (high_address as u16) << 8 | low_address as u16;
        let value = cpu.read_byte(address);
        let carry = cpu.registers.get_c() as u8;

        let result = cpu.registers.acc as u16 + value as u16 + carry as u16;
        let signed_result = (cpu.registers.acc as i8) as i16 + (value as i8) as i16 + carry as i16;

        cpu.registers.acc = result as u8;

        cpu.registers.set_c(result > 0xFF);
        cpu.registers.set_v(!(-128..=127).contains(&signed_result));
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
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
    |cpu| {
        /* 0x65 */
        /* ADC nn */
        let nn = cpu.fetch_byte();
        let value = cpu.read_byte(nn as u16);
        let carry = cpu.registers.get_c() as u8;

        let result = cpu.registers.acc as u16 + value as u16 + carry as u16;
        let signed_result = (cpu.registers.acc as i8) as i16 + (value as i8) as i16 + carry as i16;

        cpu.registers.acc = result as u8;

        cpu.registers.set_c(result > 0xFF);
        cpu.registers.set_v(!(-128..=127).contains(&signed_result));
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x66 */
        /* ROR nn */
        let nn = cpu.fetch_byte();
        let old_value = cpu.read_byte(nn as u16);
        let high_value = cpu.registers.get_c() as u8;
        let result = high_value | (old_value >> 1);
        cpu.write_byte(nn as u16, result);

        cpu.registers.set_c(old_value & 0b0000_0001 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(high_value != 0);
    },
    |_| {
        /* 0x67 */
    },
    |cpu| {
        /* 0x68 */
        /* PLA */
        cpu.registers.acc = cpu.pull_stack();
    },
    |cpu| {
        /* 0x69 */
        /* ADC #nn */
        let nn = cpu.fetch_byte();
        let carry = cpu.registers.get_c() as u8;

        let result = cpu.registers.acc as u16 + nn as u16 + carry as u16;
        let signed_result = (cpu.registers.acc as i8) as i16 + (nn as i8) as i16 + carry as i16;

        cpu.registers.acc = result as u8;

        cpu.registers.set_c(result > 0xFF);
        cpu.registers.set_v(!(-128..=127).contains(&signed_result));
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x6A */
        /* ROR A */
        let old_value = cpu.registers.acc;
        let high_value = cpu.registers.get_c() as u8;
        cpu.registers.acc = high_value | (old_value >> 1);

        cpu.registers.set_c(old_value & 0b0000_0001 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(high_value != 0);
    },
    |_| {
        /* 0x6B */
    },
    |cpu| {
        /* 0x6C */
        /* JMP (nnnn) */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = (high_nn as u16) << 8 | low_nn as u16;
        let low_value = cpu.read_byte(address);
        let high_value = cpu.read_byte(address + 1);
        cpu.registers.pc = (high_value as u16) << 8 | low_value as u16;
    },
    |cpu| {
        /* 0x6D */
        /* ADC nnnn */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = (high_nn as u16) << 8 | low_nn as u16;
        let value = cpu.read_byte(address);
        let carry = cpu.registers.get_c() as u8;

        let result = cpu.registers.acc as u16 + value as u16 + carry as u16;
        let signed_result = (cpu.registers.acc as i8) as i16 + (value as i8) as i16 + carry as i16;

        cpu.registers.acc = result as u8;

        cpu.registers.set_c(result > 0xFF);
        cpu.registers.set_v(!(-128..=127).contains(&signed_result));
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x6E */
        /* ROR nnnn */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = (high_nn as u16) << 8 | low_nn as u16;
        let old_value = cpu.read_byte(address);
        let high_value = cpu.registers.get_c() as u8;
        let result = high_value | (old_value >> 1);
        cpu.write_byte(address, result);

        cpu.registers.set_c(old_value & 0b0000_0001 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(high_value != 0);
    },
    |_| {
        /* 0x6F */
    },
    |cpu| {
        /* 0x70 */
        /* BVS */
        if cpu.registers.get_v() {
            cpu.registers.pc = cpu.registers.pc.wrapping_add_signed(cpu.fetch_byte() as i8 as i16);
        }
    },
    |cpu| {
        /* 0x71 */
        /* ADC (nn),Y */
        let nn = cpu.fetch_byte();
        let low_address = cpu.read_byte(nn as u16);
        let high_address = cpu.read_byte((nn + 1) as u16);
        let address = ((high_address as u16) << 8 | low_address as u16) + cpu.registers.y as u16;
        let value = cpu.read_byte(address);
        let carry = cpu.registers.get_c() as u8;

        let result = cpu.registers.acc as u16 + value as u16 + carry as u16;
        let signed_result = (cpu.registers.acc as i8) as i16 + (value as i8) as i16 + carry as i16;

        cpu.registers.acc = result as u8;

        cpu.registers.set_c(result > 0xFF);
        cpu.registers.set_v(!(-128..=127).contains(&signed_result));
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
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
    |cpu| {
        /* 0x75 */
        /* ADC nn,X */
        let nn = cpu.fetch_byte();
        let address = nn.wrapping_add(cpu.registers.x);
        let value = cpu.read_byte(address as u16);
        let carry = cpu.registers.get_c() as u8;

        let result = cpu.registers.acc as u16 + value as u16 + carry as u16;
        let signed_result = (cpu.registers.acc as i8) as i16 + (value as i8) as i16 + carry as i16;

        cpu.registers.acc = result as u8;

        cpu.registers.set_c(result > 0xFF);
        cpu.registers.set_v(!(-128..=127).contains(&signed_result));
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x76 */
        /* ROR nn,X */
        let nn = cpu.fetch_byte();
        let address = nn.wrapping_add(cpu.registers.x);
        let old_value = cpu.read_byte(address as u16);
        let high_value = cpu.registers.get_c() as u8;
        let result = high_value | (old_value >> 1);
        cpu.write_byte(address as u16, result);

        cpu.registers.set_c(old_value & 0b0000_0001 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(high_value != 0);
    },
    |_| {
        /* 0x77 */
    },
    |cpu| {
        /* 0x78 */
        /* SEI */
        cpu.registers.set_i(true);
    },
    |cpu| {
        /* 0x79 */
        /* ADC nnnn,Y */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = ((high_nn as u16) << 8 | low_nn as u16) + cpu.registers.y as u16;
        let value = cpu.read_byte(address);
        let carry = cpu.registers.get_c() as u8;

        let result = cpu.registers.acc as u16 + value as u16 + carry as u16;
        let signed_result = (cpu.registers.acc as i8) as i16 + (value as i8) as i16 + carry as i16;

        cpu.registers.acc = result as u8;

        cpu.registers.set_c(result > 0xFF);
        cpu.registers.set_v(!(-128..=127).contains(&signed_result));
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
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
    |cpu| {
        /* 0x7D */
        /* ADC nnnn,X */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = ((high_nn as u16) << 8 | low_nn as u16) + cpu.registers.x as u16;
        let value = cpu.read_byte(address);
        let carry = cpu.registers.get_c() as u8;

        let result = cpu.registers.acc as u16 + value as u16 + carry as u16;
        let signed_result = (cpu.registers.acc as i8) as i16 + (value as i8) as i16 + carry as i16;

        cpu.registers.acc = result as u8;

        cpu.registers.set_c(result > 0xFF);
        cpu.registers.set_v(!(-128..=127).contains(&signed_result));
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x7E */
        /* ROR nnnn,X */
        let low_nn = cpu.fetch_byte();
        let high_nn = cpu.fetch_byte();
        let address = ((high_nn as u16) << 8 | low_nn as u16) + cpu.registers.x as u16;
        let old_value = cpu.read_byte(address);
        let high_value = cpu.registers.get_c() as u8;
        let result = high_value | (old_value >> 1);
        cpu.write_byte(address, result);

        cpu.registers.set_c(old_value & 0b0000_0001 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(high_value != 0);
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