use crate::cpu::Cpu;

fn indirect_x(cpu: &mut Cpu) -> u16 {
    let nn = cpu.fetch_byte();
    let _ = cpu.read_byte(u16::from(nn));
    let low_address = cpu.read_byte(u16::from(nn.wrapping_add(cpu.registers.x)));
    let high_address = cpu.read_byte(u16::from(nn.wrapping_add(cpu.registers.x).wrapping_add(1)));
    u16::from(high_address) << 8 | u16::from(low_address)
}

fn indirect_y(cpu: &mut Cpu, force_dummy: bool) -> u16 {
    let nn = cpu.fetch_byte();
    let low_address = cpu.read_byte(u16::from(nn));
    let high_address = cpu.read_byte(u16::from(nn.wrapping_add(1)));
    let page = u16::from(high_address) << 8;
    let address = (u16::from(high_address) << 8 | u16::from(low_address)).wrapping_add(u16::from(cpu.registers.y));

    if (page != address & 0xFF00) || force_dummy {
        cpu.read_byte((page) | (address & 0xFF));
    }

    address
}

fn zpg(cpu: &mut Cpu) -> u16 {
    u16::from(cpu.fetch_byte())
}

fn zpg_x(cpu: &mut Cpu) -> u16 {
    let nn = cpu.fetch_byte();
    let _ = cpu.read_byte(u16::from(nn));
    u16::from(nn.wrapping_add(cpu.registers.x))
}

fn zpg_y(cpu: &mut Cpu) -> u16 {
    let nn = cpu.fetch_byte();
    let _ = cpu.read_byte(u16::from(nn));
    u16::from(nn.wrapping_add(cpu.registers.y))
}

fn immediate(cpu: &mut Cpu) -> u8 {
    cpu.fetch_byte()
}

fn absolute(cpu: &mut Cpu) -> u16 {
    let low_nn = cpu.fetch_byte();
    let high_nn = cpu.fetch_byte();
    u16::from(high_nn) << 8 | u16::from(low_nn)
}

fn absolute_x(cpu: &mut Cpu, force_dummy: bool) -> u16 {
    let low_nn = cpu.fetch_byte();
    let high_nn = cpu.fetch_byte();
    let page = u16::from(high_nn) << 8;
    let address = (u16::from(high_nn) << 8 | u16::from(low_nn)).wrapping_add(u16::from(cpu.registers.x));

    if (page != address & 0xFF00) || force_dummy {
        cpu.read_byte((page) | (address & 0xFF));
    }

    address
}

fn absolute_y(cpu: &mut Cpu, force_dummy: bool) -> u16 {
    let low_nn = cpu.fetch_byte();
    let high_nn = cpu.fetch_byte();
    let page = u16::from(high_nn) << 8;
    let address = (u16::from(high_nn) << 8 | u16::from(low_nn)).wrapping_add(u16::from(cpu.registers.y));

    if (page != address & 0xFF00) || force_dummy {
        cpu.read_byte((page) | (address & 0xFF));
    }

    address
}

fn branch(cpu: &mut Cpu, nn: i16) {
    let _ = cpu.read_byte(cpu.registers.pc);
    let previous_page = cpu.registers.pc & 0xFF00;
    cpu.registers.pc = cpu.registers.pc.wrapping_add_signed(nn);
    if cpu.registers.pc & 0xFF00 != previous_page {
        if nn > 0 {
            let _ = cpu.read_byte(cpu.registers.pc.wrapping_sub(0x100));
        } else {
            let _ = cpu.read_byte(cpu.registers.pc.wrapping_add(0x100));
        }
    }
}

fn ora(cpu: &mut Cpu, value: u8) {
    cpu.registers.acc |= value;

    cpu.registers.set_z(cpu.registers.acc == 0);
    cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
}

fn and(cpu: &mut Cpu, value: u8) -> u8 {
    cpu.registers.acc &= value;

    cpu.registers.set_z(cpu.registers.acc == 0);
    cpu.registers.set_n(cpu.registers.acc >> 7 == 1);

    cpu.registers.acc
}

fn eor(cpu: &mut Cpu, value: u8) {
    cpu.registers.acc ^= value;

    cpu.registers.set_z(cpu.registers.acc == 0);
    cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
}

fn asl(cpu: &mut Cpu, address: u16) -> u8 {
    let old_value = cpu.read_byte(address);
    let result = old_value << 1;
    cpu.write_byte(address, old_value);
    cpu.write_byte(address, result);

    cpu.registers.set_c(old_value >> 7 == 1);
    cpu.registers.set_z(result == 0);
    cpu.registers.set_n(result >> 7 == 1);

    result
}

fn rol(cpu: &mut Cpu, address: u16) -> u8 {
    let old_value = cpu.read_byte(address);
    let low_value = u8::from(cpu.registers.get_c());
    let result = (old_value << 1) | low_value;
    cpu.write_byte(address, old_value);
    cpu.write_byte(address, result);

    cpu.registers.set_c(old_value >> 7 == 1);
    cpu.registers.set_z(result == 0);
    cpu.registers.set_n(result >> 7 == 1);

    result
}

fn lsr(cpu: &mut Cpu, address: u16) -> u8 {
    let old_value = cpu.read_byte(address);
    let result = old_value >> 1;
    cpu.write_byte(address, old_value);
    cpu.write_byte(address, result);

    cpu.registers.set_c(old_value & 0b0000_0001 == 1);
    cpu.registers.set_z(result == 0);
    cpu.registers.set_n(false);

    result
}

fn ror(cpu: &mut Cpu, address: u16) -> u8 {
    let old_value = cpu.read_byte(address);
    let high_value = u8::from(cpu.registers.get_c());
    let result = (high_value << 7) | (old_value >> 1);
    cpu.write_byte(address, old_value);
    cpu.write_byte(address, result);

    cpu.registers.set_c(old_value & 0b0000_0001 == 1);
    cpu.registers.set_z(result == 0);
    cpu.registers.set_n(high_value != 0);

    result
}

#[allow(clippy::cast_possible_wrap)]
fn adc(cpu: &mut Cpu, value: u8) {
    let mut carry = u8::from(cpu.registers.get_c());

    let mut result = u16::from(cpu.registers.acc) + u16::from(value) + u16::from(carry);
    let signed_result = i16::from(cpu.registers.acc as i8) + i16::from(value as i8) + i16::from(carry);

    cpu.registers.set_z(result.trailing_zeros() >= 8);

    if cpu.registers.get_d() {
        let mut tmp = u16::from((cpu.registers.acc & 0x0F) + (value & 0x0F) + carry);

        if tmp > 0x09 {
            tmp += 0x06;
            cpu.registers.set_c(true);
            carry = 1;
        } else {
            cpu.registers.set_c(false);
            carry = 0;
        }

        tmp = (u16::from(cpu.registers.acc) & 0xF0) + (u16::from(value) & 0xF0) + (u16::from(carry) << 4) + (tmp & 0x0F);

        cpu.registers.set_n(tmp & 0x80 != 0);
        let v = !(cpu.registers.acc ^ value) & (cpu.registers.acc ^ (tmp & 0xFF) as u8) & 0x80;
        cpu.registers.set_v(v != 0);

        if tmp > 0x9F {
            tmp += 0x60;
            cpu.registers.set_c(true);
        } else {
            cpu.registers.set_c(false);
        }

        result = tmp;
    } else {
        cpu.registers.set_c(result > 0xFF);
        cpu.registers.set_n(result & 0x80 != 0);
        cpu.registers.set_v(!(-128..=127).contains(&signed_result));
    }

    cpu.registers.acc = (result & 0xFF) as u8;
}

#[allow(clippy::cast_possible_wrap)]
fn sbc(cpu: &mut Cpu, value: u8) {
    let mut carry = u8::from(cpu.registers.get_c());

    let mut result = u16::from(cpu.registers.acc) + u16::from(value) + u16::from(carry);
    let signed_result = i16::from(cpu.registers.acc as i8) + i16::from(value as i8) + i16::from(carry);

    cpu.registers.set_z(result.trailing_zeros() >= 8);

    if cpu.registers.get_d() {
        let mut tmp = u16::from((cpu.registers.acc & 0x0F) + (value & 0x0F) + carry);

        if tmp <= 0x0f {
            tmp = tmp.wrapping_sub(0x6);
            cpu.registers.set_c(false);
            carry = 0;
        } else {
            cpu.registers.set_c(true);
            carry = 1;
        }

        tmp = (u16::from(cpu.registers.acc) & 0xF0) + (u16::from(value) & 0xF0) + (u16::from(carry) << 4) + (tmp & 0x0F);

        cpu.registers.set_n(tmp & 0x80 != 0);
        let v = !(cpu.registers.acc ^ value) & (cpu.registers.acc ^ (tmp & 0xFF) as u8) & 0x80;
        cpu.registers.set_v(v != 0);

        if tmp <= 0xFF {
            tmp = tmp.wrapping_sub(0x60);
            cpu.registers.set_c(false);
        } else {
            cpu.registers.set_c(true);
        }

        result = tmp;
    } else {
        cpu.registers.set_c(result & 0x100 != 0);
        cpu.registers.set_v(!(-128..=127).contains(&signed_result));
        cpu.registers.set_n(result & 0x80 != 0);
    }

    cpu.registers.acc = (result & 0xFF) as u8;
}

fn lda(cpu: &mut Cpu, value: u8) {
    cpu.registers.acc = value;

    cpu.registers.set_z(cpu.registers.acc == 0);
    cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
}

fn ldx(cpu: &mut Cpu, value: u8) {
    cpu.registers.x = value;

    cpu.registers.set_z(cpu.registers.x == 0);
    cpu.registers.set_n(cpu.registers.x >> 7 == 1);
}

fn ldy(cpu: &mut Cpu, value: u8) {
    cpu.registers.y = value;

    cpu.registers.set_z(cpu.registers.y == 0);
    cpu.registers.set_n(cpu.registers.y >> 7 == 1);
}

fn cmp(cpu: &mut Cpu, value: u8) {
    let result = cpu.registers.acc.wrapping_sub(value);

    cpu.registers.set_c(cpu.registers.acc >= value);
    cpu.registers.set_z(cpu.registers.acc == value);
    cpu.registers.set_n(result >> 7 == 1);
}

fn cpy(cpu: &mut Cpu, value: u8) {
    let result = cpu.registers.y.wrapping_sub(value);

    cpu.registers.set_c(cpu.registers.y >= value);
    cpu.registers.set_z(cpu.registers.y == value);
    cpu.registers.set_n(result >> 7 == 1);
}

fn cpx(cpu: &mut Cpu, value: u8) {
    let result = cpu.registers.x.wrapping_sub(value);

    cpu.registers.set_c(cpu.registers.x >= value);
    cpu.registers.set_z(cpu.registers.x == value);
    cpu.registers.set_n(result >> 7 == 1);
}

fn dec(cpu: &mut Cpu, address: u16) -> u8 {
    let value = cpu.read_byte(address);
    let result = value.wrapping_sub(1);
    cpu.write_byte(address, value);
    cpu.write_byte(address, result);

    cpu.registers.set_z(result == 0);
    cpu.registers.set_n(result >> 7 == 1);

    result
}

fn inc(cpu: &mut Cpu, address: u16) {
    let value = cpu.read_byte(address);
    let result = value.wrapping_add(1);
    cpu.write_byte(address, value);
    cpu.write_byte(address, result);

    cpu.registers.set_z(result == 0);
    cpu.registers.set_n(result >> 7 == 1);
}

fn jam(cpu: &mut Cpu) {
    cpu.read_byte(cpu.registers.pc);
    cpu.read_byte(0xFFFF);
    cpu.read_byte(0xFFFE);
    cpu.read_byte(0xFFFE);
    cpu.read_byte(0xFFFF);
    cpu.read_byte(0xFFFF);
    cpu.read_byte(0xFFFF);
    cpu.read_byte(0xFFFF);
    cpu.read_byte(0xFFFF);
    cpu.read_byte(0xFFFF);
}

#[allow(clippy::cast_possible_wrap)]
pub static OPCODES: [fn(&mut Cpu); 0x100] = {
    [|cpu| {
        /* 0x00 */
        /* BRK */
        let _ = cpu.fetch_byte();
        cpu.push_stack((cpu.registers.pc >> 8) as u8);
        cpu.push_stack((cpu.registers.pc & 0xFF) as u8);
        cpu.push_stack(cpu.registers.p | 0b0001_0000);
        cpu.registers.set_i(true);
        let low_value = cpu.read_byte(0xFFFE);
        let high_value = cpu.read_byte(0xFFFE + 1);
        let value = u16::from(high_value) << 8 | u16::from(low_value);
        cpu.registers.pc = value;
    },
    |cpu| {
        /* 0x01 */
        /* ORA (nn,X) */
        let address = indirect_x(cpu);
        let value = cpu.read_byte(address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x02 */
        /* JAM */
        jam(cpu);
    },
    |cpu| {
        /* 0x03 */
        /* SLO (nn,X) */
        let address = indirect_x(cpu);
        let value = asl(cpu, address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x04 */
        /* NOP nn */
        let address = zpg(cpu);
        cpu.read_byte(address);
    },
    |cpu| {
        /* 0x05 */
        /* ORA nn */
        let address = zpg(cpu);
        let value = cpu.read_byte(address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x06 */
        /* ASL nn */
        let address = zpg(cpu);
        asl(cpu, address);
    },
    |cpu| {
        /* 0x07 */
        /* SLO nn */
        let address = zpg(cpu);
        let value = asl(cpu, address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x08 */
        /* PHP */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.push_stack(cpu.registers.p | 0b0001_0000);
    },
    |cpu| {
        /* 0x09 */
        /* ORA #nn */
        let value = immediate(cpu);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x0A */
        /* ASL A */
        let _ = cpu.read_byte(cpu.registers.pc);
        let old_value = cpu.registers.acc;
        cpu.registers.acc <<= 1;

        cpu.registers.set_c(old_value >> 7 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x0B */
        /* ANC #nn */
        let value = immediate(cpu);
        let result = and(cpu, value);

        cpu.registers.set_c(result >> 7 == 1);
    },
    |cpu| {
        /* 0x0C */
        /* NOP nnnn */
        let address = absolute(cpu);
        cpu.read_byte(address);
    },
    |cpu| {
        /* 0x0D */
        /* ORA nnnn */
        let address = absolute(cpu);
        let value = cpu.read_byte(address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x0E */
        /* ASL nnnn */
        let address = absolute(cpu);
        asl(cpu, address);
    },
    |cpu| {
        /* 0x0F */
        /* SLO nnnn */
        let address = absolute(cpu);
        let value = asl(cpu, address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x10 */
        /* BPL */
        let nn = i16::from(cpu.fetch_byte() as i8);
        if !cpu.registers.get_n() {
            branch(cpu, nn);
        }
    },
    |cpu| {
        /* 0x11 */
        /* ORA (nn),Y */
        let address = indirect_y(cpu, false);
        let value = cpu.read_byte(address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x12 */
        /* JAM */
        jam(cpu)
    },
    |cpu| {
        /* 0x13 */
        /* SLO (nn),Y */
        let address = indirect_y(cpu, true);
        let value = asl(cpu, address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x14 */
        /* NOP nn,X */
        let address = zpg_x(cpu);
        cpu.read_byte(address);
    },
    |cpu| {
        /* 0x15 */
        /* ORA nn,X */
        let address = zpg_x(cpu);
        let value = cpu.read_byte(address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x16 */
        /* ASL nn,X */
        let address = zpg_x(cpu);
        asl(cpu, address);
    },
    |cpu| {
        /* 0x17 */
        /* SLO nn,X */
        let address = zpg_x(cpu);
        let value = asl(cpu, address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x18 */
        /* CLC */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.set_c(false);
    },
    |cpu| {
        /* 0x19 */
        /* ORA nnnn,Y */
        let address = absolute_y(cpu, false);
        let value = cpu.read_byte(address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x1A */
        /* NOP */
        cpu.read_byte(cpu.registers.pc);
    },
    |cpu| {
        /* 0x1B */
        /* SLO nnnn,Y */
        let address = absolute_y(cpu, true);
        let value = asl(cpu, address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x1C */
        /* NOP nnnn,X */
        let address = absolute_x(cpu, false);
        cpu.read_byte(address);
    },
    |cpu| {
        /* 0x1D */
        /* ORA nnnn,X */
        let address = absolute_x(cpu, false);
        let value = cpu.read_byte(address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x1E */
        /* ASL nnnn,X */
        let address = absolute_x(cpu, true);
        asl(cpu, address);
    },
    |cpu| {
        /* 0x1F */
        /* SLO nnnn,X */
        let address = absolute_x(cpu, true);
        let value = asl(cpu, address);
        ora(cpu, value);
    },
    |cpu| {
        /* 0x20 */
        /* JSR nnnn */
        let low_value = cpu.fetch_byte();
        let _ = cpu.read_byte(0x100 + u16::from(cpu.registers.sp));
        cpu.push_stack((cpu.registers.pc >> 8) as u8);
        cpu.push_stack((cpu.registers.pc & 0xFF) as u8);
        let high_value = cpu.read_byte(cpu.registers.pc);
        cpu.registers.pc = u16::from(high_value) << 8 | u16::from(low_value);
    },
    |cpu| {
        /* 0x21 */
        /* AND (nn,X) */
        let address = indirect_x(cpu);
        let value = cpu.read_byte(address);
        and(cpu, value);
    },
    |cpu| {
        /* 0x22 */
        /* JAM */
        jam(cpu);
    },
    |cpu| {
        /* 0x23 */
        /* RLA (nn,X) */
        let address = indirect_x(cpu);
        let value = rol(cpu, address);
        let _ = and(cpu, value);
    },
    |cpu| {
        /* 0x24 */
        /* BIT nn */
        let address = zpg(cpu);
        let value = cpu.read_byte(address);
        let result = cpu.registers.acc & value;

        cpu.registers.set_z(result == 0);
        cpu.registers.set_n(value >> 7 == 1);
        cpu.registers.set_v(value & 0b0100_0000 != 0);
    },
    |cpu| {
        /* 0x25 */
        /* AND nn */
        let address = zpg(cpu);
        let value = cpu.read_byte(address);
        and(cpu, value);
    },
    |cpu| {
        /* 0x26 */
        /* ROL nn */
        let address = zpg(cpu);
        rol(cpu, address);
    },
    |cpu| {
        /* 0x27 */
        /* RLA nn */
        let address = zpg(cpu);
        let value = rol(cpu, address);
        let _ = and(cpu, value);
    },
    |cpu| {
        /* 0x28 */
        /* PLP */
        let _ = cpu.read_byte(cpu.registers.pc);
        let _ = cpu.read_byte(0x100 + u16::from(cpu.registers.sp));
        cpu.registers.p = (cpu.pull_stack() | 0b0010_0000) & 0b1110_1111;
    },
    |cpu| {
        /* 0x29 */
        /* AND #nn */
        let value = immediate(cpu);
        and(cpu, value);
    },
    |cpu| {
        /* 0x2A */
        /* ROL A */
        let _ = cpu.read_byte(cpu.registers.pc);
        let old_value = cpu.registers.acc;
        let low_value = u8::from(cpu.registers.get_c());
        cpu.registers.acc = (old_value << 1) | low_value;

        cpu.registers.set_c(old_value >> 7 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x2B */
        /* ANC #nn */
        let value = immediate(cpu);
        let result = and(cpu, value);

        cpu.registers.set_c(result >> 7 == 1);
    },
    |cpu| {
        /* 0x2C */
        /* BIT nnnn */
        let address = absolute(cpu);
        let value = cpu.read_byte(address);
        let result = cpu.registers.acc & value;

        cpu.registers.set_z(result == 0);
        cpu.registers.set_n(value >> 7 == 1);
        cpu.registers.set_v(value & 0b0100_0000 != 0);
    },
    |cpu| {
        /* 0x2D */
        /* AND nnnn */
        let address = absolute(cpu);
        let value = cpu.read_byte(address);
        and(cpu, value);
    },
    |cpu| {
        /* 0x2E */
        /* ROL nnnn */
        let address = absolute(cpu);
        rol(cpu, address);
    },
    |cpu| {
        /* 0x2F */
        /* RLA nnnn */
        let address = absolute(cpu);
        let value = rol(cpu, address);
        let _ = and(cpu, value);
    },
    |cpu| {
        /* 0x30 */
        /* BMI */
        let nn = i16::from(cpu.fetch_byte() as i8);
        if cpu.registers.get_n() {
            branch(cpu, nn);
        }
    },
    |cpu| {
        /* 0x31 */
        /* AND (nn),Y */
        let address = indirect_y(cpu, false);
        let value = cpu.read_byte(address);
        and(cpu, value);
    },
    |cpu| {
        /* 0x32 */
        /* JAM */
        jam(cpu);
    },
    |cpu| {
        /* 0x33 */
        /* RLA (nn),Y */
        let address = indirect_y(cpu, true);
        let value = rol(cpu, address);
        let _ = and(cpu, value);
    },
    |cpu| {
        /* 0x34 */
        /* NOP nn,X */
        let address = zpg_x(cpu);
        cpu.read_byte(address);
    },
    |cpu| {
        /* 0x35 */
        /* AND nn,X */
        let address = zpg_x(cpu);
        let value = cpu.read_byte(address);
        and(cpu, value);
    },
    |cpu| {
        /* 0x36 */
        /* ROL nn,X */
        let address = zpg_x(cpu);
        rol(cpu, address);
    },
    |cpu| {
        /* 0x37 */
        /* RLA nn,X */
        let address = zpg_x(cpu);
        let value = rol(cpu, address);
        let _ = and(cpu, value);
    },
    |cpu| {
        /* 0x38 */
        /* SEC */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.set_c(true);
    },
    |cpu| {
        /* 0x39 */
        /* AND nnnn,Y */
        let address = absolute_y(cpu, false);
        let value = cpu.read_byte(address);
        and(cpu, value);
    },
    |cpu| {
        /* 0x3A */
        /* NOP */
        cpu.read_byte(cpu.registers.pc);
    },
    |cpu| {
        /* 0x3B */
        /* RLA nnnn,Y */
        let address = absolute_y(cpu, true);
        let value = rol(cpu, address);
        let _ = and(cpu, value);
    },
    |cpu| {
        /* 0x3C */
        /* NOP nnnn,X */
        let address = absolute_x(cpu, false);
        cpu.read_byte(address);
    },
    |cpu| {
        /* 0x3D */
        /* AND nnnn,X */
        let address = absolute_x(cpu, false);
        let value = cpu.read_byte(address);
        and(cpu, value);
    },
    |cpu| {
        /* 0x3E */
        /* ROL nnnn,X */
        let address = absolute_x(cpu, true);
        rol(cpu, address);
    },
    |cpu| {
        /* 0x3F */
        /* RLA nnnn,X */
        let address = absolute_x(cpu, true);
        let value = rol(cpu, address);
        let _ = and(cpu, value);
    },
    |cpu| {
        /* 0x40 */
        /* RTI */
        let _ = cpu.read_byte(cpu.registers.pc);
        let _ = cpu.read_byte(0x100 + u16::from(cpu.registers.sp));
        cpu.registers.p = (cpu.pull_stack() | 0b0010_0000) & 0b1110_1111;
        let low_value = u16::from(cpu.pull_stack());
        let high_value = u16::from(cpu.pull_stack());
        let value = high_value << 8 | low_value;
        cpu.registers.pc = value;
    },
    |cpu| {
        /* 0x41 */
        /* EOR (nn,X) */
        let address = indirect_x(cpu);
        let value = cpu.read_byte(address);
        eor(cpu, value);
    },
    |cpu| {
        /* 0x42 */
        /* JAM */
        jam(cpu);
    },
    |cpu| {
        /* 0x43 */
        /* SRE (nn,X) */
        let address = indirect_x(cpu);
        let value = lsr(cpu, address);
        eor(cpu, value);
    },
    |cpu| {
        /* 0x44 */
        /* NOP nn */
        let address = zpg(cpu);
        cpu.read_byte(address);
    },
    |cpu| {
        /* 0x45 */
        /* EOR nn */
        let address = zpg(cpu);
        let value = cpu.read_byte(address);
        eor(cpu, value);
    },
    |cpu| {
        /* 0x46 */
        /* LSR nn */
        let address = zpg(cpu);
        lsr(cpu, address);
    },
    |cpu| {
        /* 0x47 */
        /* SRE nn */
        let address = zpg(cpu);
        let value = lsr(cpu, address);
        eor(cpu, value);
    },
    |cpu| {
        /* 0x48 */
        /* PHA */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.push_stack(cpu.registers.acc);
    },
    |cpu| {
        /* 0x49 */
        /* EOR #nn */
        let value = immediate(cpu);
        eor(cpu, value);
    },
    |cpu| {
        /* 0x4A */
        /* LSR A */
        let _ = cpu.read_byte(cpu.registers.pc);
        let old_value = cpu.registers.acc;
        cpu.registers.acc >>= 1;

        cpu.registers.set_c(old_value & 0b0000_0001 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(false);
    },
    |cpu| {
        /* 0x4B */
        /* ALR #nn */
        let value = immediate(cpu);
        let _ = and(cpu, value);
        let old_value = cpu.registers.acc;
        cpu.registers.acc >>= 1;

        cpu.registers.set_c(old_value & 0b0000_0001 == 1);
        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(false);
    },
    |cpu| {
        /* 0x4C */
        /* JMP nnnn */
        let address = absolute(cpu);
        cpu.registers.pc = address;
    },
    |cpu| {
        /* 0x4D */
        /* EOR nnnn */
        let address = absolute(cpu);
        let value = cpu.read_byte(address);
        eor(cpu, value);
    },
    |cpu| {
        /* 0x4E */
        /* LSR nnnn */
        let address = absolute(cpu);
        lsr(cpu, address);
    },
    |cpu| {
        /* 0x4F */
        /* SRE nnnn */
        let address = absolute(cpu);
        let value = lsr(cpu, address);
        eor(cpu, value);
    },
    |cpu| {
        /* 0x50 */
        /* BVC */
        let nn = i16::from(cpu.fetch_byte() as i8);
        if !cpu.registers.get_v() {
            branch(cpu, nn);
        }
    },
    |cpu| {
        /* 0x51 */
        /* EOR (nn),Y */
        let address = indirect_y(cpu, false);
        let value = cpu.read_byte(address);
        eor(cpu, value);
    },
    |cpu| {
        /* 0x52 */
        /* JAM */
        jam(cpu)
    },
    |cpu| {
        /* 0x53 */
        /* SRE (nn),Y */
        let address = indirect_y(cpu, true);
        let value = lsr(cpu, address);
        eor(cpu, value);
    },
    |cpu| {
        /* 0x54 */
        /* NOP nn,X */
        let address = zpg_x(cpu);
        cpu.read_byte(address);
    },
    |cpu| {
        /* 0x55 */
        /* EOR nn,X */
        let address = zpg_x(cpu);
        let value = cpu.read_byte(address);
        eor(cpu, value);
    },
    |cpu| {
        /* 0x56 */
        /* LSR nn,X */
        let address = zpg_x(cpu);
        lsr(cpu, address);
    },
    |_| {
        /* 0x57 */
    },
    |cpu| {
        /* 0x58 */
        /* CLI */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.set_i(false);
    },
    |cpu| {
        /* 0x59 */
        /* EOR nnnn,Y */
        let address = absolute_y(cpu, false);
        let value = cpu.read_byte(address);
        eor(cpu, value);
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
        let address = absolute_x(cpu, false);
        let value = cpu.read_byte(address);
        eor(cpu, value);
    },
    |cpu| {
        /* 0x5E */
        /* LSR nnnn,X */
        let address = absolute_x(cpu, true);
        lsr(cpu, address);
    },
    |_| {
        /* 0x5F */
    },
    |cpu| {
        /* 0x60 */
        /* RTS */
        let _ = cpu.read_byte(cpu.registers.pc);
        let _ = cpu.read_byte(0x100 + u16::from(cpu.registers.sp));
        let low_address = cpu.pull_stack();
        let high_address = cpu.pull_stack();
        let new_pc = u16::from(high_address) << 8 | u16::from(low_address);
        let _ = cpu.read_byte(new_pc);
        cpu.registers.pc = new_pc + 1;
    },
    |cpu| {
        /* 0x61 */
        /* ADC (nn,X) */
        let address = indirect_x(cpu);
        let value = cpu.read_byte(address);
        adc(cpu, value);
    },
    |cpu| {
        /* 0x62 */
        /* JAM */
        jam(cpu)
    },
    |cpu| {
        /* 0x63 */
        /* RRA (nn,X) */
        let address = indirect_x(cpu);
        let value = ror(cpu, address);
        adc(cpu, value);
    },
    |cpu| {
        /* 0x64 */
        /* NOP nn */
        let address = zpg(cpu);
        cpu.read_byte(address);
    },
    |cpu| {
        /* 0x65 */
        /* ADC nn */
        let address = zpg(cpu);
        let value = cpu.read_byte(address);
        adc(cpu, value);
    },
    |cpu| {
        /* 0x66 */
        /* ROR nn */
        let address = zpg(cpu);
        ror(cpu, address);
    },
    |_| {
        /* 0x67 */
    },
    |cpu| {
        /* 0x68 */
        /* PLA */
        let _ = cpu.read_byte(cpu.registers.pc);
        let _ = cpu.read_byte(0x100 + u16::from(cpu.registers.sp));
        cpu.registers.acc = cpu.pull_stack();

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x69 */
        /* ADC #nn */
        let value = immediate(cpu);
        adc(cpu, value);
    },
    |cpu| {
        /* 0x6A */
        /* ROR A */
        let _ = cpu.read_byte(cpu.registers.pc);
        let old_value = cpu.registers.acc;
        let high_value = u8::from(cpu.registers.get_c());
        cpu.registers.acc = (high_value << 7) | (old_value >> 1);

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
        let address = u16::from(high_nn) << 8 | u16::from(low_nn);
        let low_value = cpu.read_byte(address);
        let high_value = cpu.read_byte((address & 0xFF00) + ((address.wrapping_add(1)) & 0xFF));
        cpu.registers.pc = u16::from(high_value) << 8 | u16::from(low_value);
    },
    |cpu| {
        /* 0x6D */
        /* ADC nnnn */
        let address = absolute(cpu);
        let value = cpu.read_byte(address);
        adc(cpu, value);
    },
    |cpu| {
        /* 0x6E */
        /* ROR nnnn */
        let address = absolute(cpu);
        ror(cpu, address);
    },
    |_| {
        /* 0x6F */
    },
    |cpu| {
        /* 0x70 */
        /* BVS */
        let nn = i16::from(cpu.fetch_byte() as i8);
        if cpu.registers.get_v() {
            branch(cpu, nn);
        }
    },
    |cpu| {
        /* 0x71 */
        /* ADC (nn),Y */
        let address = indirect_y(cpu, false);
        let value = cpu.read_byte(address);
        adc(cpu, value);
    },
    |cpu| {
        /* 0x72 */
        /* JAM */
        jam(cpu)
    },
    |cpu| {
        /* 0x73 */
        /* RRA (nn),Y */
        let address = indirect_y(cpu, true);
        let value = ror(cpu, address);
        adc(cpu, value);
    },
    |cpu| {
        /* 0x74 */
        /* NOP nn,X */
        let address = zpg_x(cpu);
        cpu.read_byte(address);
    },
    |cpu| {
        /* 0x75 */
        /* ADC nn,X */
        let address = zpg_x(cpu);
        let value = cpu.read_byte(address);
        adc(cpu, value);
    },
    |cpu| {
        /* 0x76 */
        /* ROR nn,X */
        let address = zpg_x(cpu);
        ror(cpu, address);
    },
    |_| {
        /* 0x77 */
    },
    |cpu| {
        /* 0x78 */
        /* SEI */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.set_i(true);
    },
    |cpu| {
        /* 0x79 */
        /* ADC nnnn,Y */
        let address = absolute_y(cpu, false);
        let value = cpu.read_byte(address);
        adc(cpu, value);
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
        let address = absolute_x(cpu, false);
        let value = cpu.read_byte(address);
        adc(cpu, value);
    },
    |cpu| {
        /* 0x7E */
        /* ROR nnnn,X */
        let address = absolute_x(cpu, true);
        ror(cpu, address);
    },
    |_| {
        /* 0x7F */
    },
    |cpu| {
        /* 0x80 */
        /* NOP #nn */
        let _ = immediate(cpu);
    },
    |cpu| {
        /* 0x81 */
        /* STA (nn,X) */
        let address = indirect_x(cpu);
        cpu.write_byte(address, cpu.registers.acc);
    },
    |cpu| {
        /* 0x82 */
        /* NOP #nn */
        let _ = immediate(cpu);
    },
    |cpu| {
        /* 0x83 */
        /* SAX (nn, X) */
        let address = indirect_x(cpu);
        let value = cpu.registers.acc & cpu.registers.x;
        cpu.write_byte(address, value);
    },
    |cpu| {
        /* 0x84 */
        /* STY nn */
        let address = zpg(cpu);
        cpu.write_byte(address, cpu.registers.y);
    },
    |cpu| {
        /* 0x85 */
        /* STA nn */
        let address = zpg(cpu);
        cpu.write_byte(address, cpu.registers.acc);
    },
    |cpu| {
        /* 0x86 */
        /* STX nn */
        let address = zpg(cpu);
        cpu.write_byte(address, cpu.registers.x);
    },
    |_| {
        /* 0x87 */
    },
    |cpu| {
        /* 0x88 */
        /* DEY */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.y = cpu.registers.y.wrapping_sub(1);

        cpu.registers.set_z(cpu.registers.y == 0);
        cpu.registers.set_n(cpu.registers.y >> 7 == 1);
    },
    |cpu| {
        /* 0x89 */
        /* NOP #nn */
        let _ = immediate(cpu);
    },
    |cpu| {
        /* 0x8A */
        /* TXA */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.acc = cpu.registers.x;

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |_| {
        /* 0x8B */
    },
    |cpu| {
        /* 0x8C */
        /* STY nnnn */
        let address = absolute(cpu);
        cpu.write_byte(address, cpu.registers.y);
    },
    |cpu| {
        /* 0x8D */
        /* STA nnnn */
        let address = absolute(cpu);
        cpu.write_byte(address, cpu.registers.acc);
    },
    |cpu| {
        /* 0x8E */
        /* STX nnnn */
        let address = absolute(cpu);
        cpu.write_byte(address, cpu.registers.x);
    },
    |_| {
        /* 0x8F */
    },
    |cpu| {
        /* 0x90 */
        /* BCC */
        let nn = i16::from(cpu.fetch_byte() as i8);
        if !cpu.registers.get_c() {
            branch(cpu, nn);
        }
    },
    |cpu| {
        /* 0x91 */
        /* STA (nn),Y */
        let address = indirect_y(cpu, true);
        cpu.write_byte(address, cpu.registers.acc);
    },
    |cpu| {
        /* 0x92 */
        /* JAM */
        jam(cpu)
    },
    |_| {
        /* 0x93 */
    },
    |cpu| {
        /* 0x94 */
        /* STY nn,X */
        let address = zpg_x(cpu);
        cpu.write_byte(address, cpu.registers.y);
    },
    |cpu| {
        /* 0x95 */
        /* STA nn,X */
        let address = zpg_x(cpu);
        cpu.write_byte(address, cpu.registers.acc);
    },
    |cpu| {
        /* 0x96 */
        /* STX nn,Y */
        let address = zpg_y(cpu);
        cpu.write_byte(address, cpu.registers.x);
    },
    |_| {
        /* 0x97 */
    },
    |cpu| {
        /* 0x98 */
        /* TYA */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.acc = cpu.registers.y;

        cpu.registers.set_z(cpu.registers.acc == 0);
        cpu.registers.set_n(cpu.registers.acc >> 7 == 1);
    },
    |cpu| {
        /* 0x99 */
        /* STA nnnn,Y */
        let address = absolute_y(cpu, true);
        cpu.write_byte(address, cpu.registers.acc);
    },
    |cpu| {
        /* 0x9A */
        /* TXS */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.sp = cpu.registers.x;
    },
    |_| {
        /* 0x9B */
    },
    |_| {
        /* 0x9C */
    },
    |cpu| {
        /* 0x9D */
        /* STA nnnn,X */
        let address = absolute_x(cpu, true);
        cpu.write_byte(address, cpu.registers.acc);
    },
    |_| {
        /* 0x9E */
    },
    |_| {
        /* 0x9F */
    },
    |cpu| {
        /* 0xA0 */
        /* LDY #nn */
        let value = immediate(cpu);
        ldy(cpu, value);
    },
    |cpu| {
        /* 0xA1 */
        /* LDA (nn,X) */
        let address = indirect_x(cpu);
        let value = cpu.read_byte(address);
        lda(cpu, value);
    },
    |cpu| {
        /* 0xA2 */
        /* LDX #nn */
        let value = immediate(cpu);
        ldx(cpu, value);
    },
    |_| {
        /* 0xA3 */
    },
    |cpu| {
        /* 0xA4 */
        /* LDY nn */
        let address = zpg(cpu);
        let value = cpu.read_byte(address);
        ldy(cpu, value);
    },
    |cpu| {
        /* 0xA5 */
        /* LDA nn */
        let address = zpg(cpu);
        let value = cpu.read_byte(address);
        lda(cpu, value);
    },
    |cpu| {
        /* 0xA6 */
        /* LDX nn */
        let address = zpg(cpu);
        let value = cpu.read_byte(address);
        ldx(cpu, value);
    },
    |_| {
        /* 0xA7 */
    },
    |cpu| {
        /* 0xA8 */
        /* TAY */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.y = cpu.registers.acc;

        cpu.registers.set_z(cpu.registers.y == 0);
        cpu.registers.set_n(cpu.registers.y >> 7 == 1);
    },
    |cpu| {
        /* 0xA9 */
        /* LDA #nn */
        let value = immediate(cpu);
        lda(cpu, value);
    },
    |cpu| {
        /* 0xAA */
        /* TAX */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.x = cpu.registers.acc;

        cpu.registers.set_z(cpu.registers.x == 0);
        cpu.registers.set_n(cpu.registers.x >> 7 == 1);
    },
    |_| {
        /* 0xAB */
    },
    |cpu| {
        /* 0xAC */
        /* LDY nnnn */
        let address = absolute(cpu);
        let value = cpu.read_byte(address);
        ldy(cpu, value);
    },
    |cpu| {
        /* 0xAD */
        /* LDA nnnn */
        let address = absolute(cpu);
        let value = cpu.read_byte(address);
        lda(cpu, value);
    },
    |cpu| {
        /* 0xAE */
        /* LDX nnnn */
        let address = absolute(cpu);
        let value = cpu.read_byte(address);
        ldx(cpu, value);
    },
    |_| {
        /* 0xAF */
    },
    |cpu| {
        /* 0xB0 */
        /* BCS */
        #[allow(clippy::cast_possible_wrap)]
        let nn = i16::from(cpu.fetch_byte() as i8);
        if cpu.registers.get_c() {
            branch(cpu, nn);
        }
    },
    |cpu| {
        /* 0xB1 */
        /* LDA (nn),Y */
        let address = indirect_y(cpu, false);
        let value = cpu.read_byte(address);
        lda(cpu, value);
    },
    |cpu| {
        /* 0xB2 */
        /* JAM */
        jam(cpu)
    },
    |_| {
        /* 0xB3 */
    },
    |cpu| {
        /* 0xB4 */
        /* LDY nn,X */
        let address = zpg_x(cpu);
        let value = cpu.read_byte(address);
        ldy(cpu, value);
    },
    |cpu| {
        /* 0xB5 */
        /* LDA nn,X */
        let address = zpg_x(cpu);
        let value = cpu.read_byte(address);
        lda(cpu, value);
    },
    |cpu| {
        /* 0xB6 */
        /* LDX nn,Y */
        let address = zpg_y(cpu);
        let value = cpu.read_byte(address);
        ldx(cpu, value);
    },
    |_| {
        /* 0xB7 */
    },
    |cpu| {
        /* 0xB8 */
        /* CLV */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.set_v(false);
    },
    |cpu| {
        /* 0xB9 */
        /* LDA nnnn,Y */
        let address = absolute_y(cpu, false);
        let value = cpu.read_byte(address);
        lda(cpu, value);
    },
    |cpu| {
        /* 0xBA */
        /* TSX */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.x = cpu.registers.sp;

        cpu.registers.set_z(cpu.registers.sp == 0);
        cpu.registers.set_n(cpu.registers.sp >> 7 == 1);
    },
    |_| {
        /* 0xBB */
    },
    |cpu| {
        /* 0xBC */
        /* LDY nnnn,X */
        let address = absolute_x(cpu, false);
        let value = cpu.read_byte(address);
        ldy(cpu, value);
    },
    |cpu| {
        /* 0xBD */
        /* LDA nnnn,X */
        let address = absolute_x(cpu, false);
        let value = cpu.read_byte(address);
        lda(cpu, value);
    },
    |cpu| {
        /* 0xBE */
        /* LDX nnnn,Y */
        let address = absolute_y(cpu, false);
        let value = cpu.read_byte(address);
        ldx(cpu, value);
    },
    |_| {
        /* 0xBF */
    },
    |cpu| {
        /* 0xC0 */
        /* CPY #nn */
        let value = immediate(cpu);
        cpy(cpu, value);
    },
    |cpu| {
        /* 0xC1 */
        /* CMP (nn,X) */
        let address = indirect_x(cpu);
        let value = cpu.read_byte(address);
        cmp(cpu, value);
    },
    |cpu| {
        /* 0xC2 */
        /* NOP #nn */
        let _ = immediate(cpu);
    },
    |_| {
        /* 0xC3 */
    },
    |cpu| {
        /* 0xC4 */
        /* CPY nn */
        let address = zpg(cpu);
        let value = cpu.read_byte(address);
        cpy(cpu, value);
    },
    |cpu| {
        /* 0xC5 */
        /* CMP nn */
        let address = zpg(cpu);
        let value = cpu.read_byte(address);
        cmp(cpu, value);
    },
    |cpu| {
        /* 0xC6 */
        /* DEC nn */
        let address = zpg(cpu);
        dec(cpu, address);
    },
    |cpu| {
        /* 0xC7 */
        let address = zpg(cpu);
        let value = dec(cpu, address);
        cmp(cpu, value);
    },
    |cpu| {
        /* 0xC8 */
        /* INY */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.y = cpu.registers.y.wrapping_add(1);

        cpu.registers.set_z(cpu.registers.y == 0);
        cpu.registers.set_n(cpu.registers.y >> 7 == 1);
    },
    |cpu| {
        /* 0xC9 */
        /* CMP #nn */
        let value = immediate(cpu);
        cmp(cpu, value);
    },
    |cpu| {
        /* 0xCA */
        /* DEX */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.x = cpu.registers.x.wrapping_sub(1);

        cpu.registers.set_z(cpu.registers.x == 0);
        cpu.registers.set_n(cpu.registers.x >> 7 == 1);
    },
    |_| {
        /* 0xCB */
    },
    |cpu| {
        /* 0xCC */
        /* CPY nnnn */
        let address = absolute(cpu);
        let value = cpu.read_byte(address);
        cpy(cpu, value);
    },
    |cpu| {
        /* 0xCD */
        /* CMP nnnn */
        let address = absolute(cpu);
        let value = cpu.read_byte(address);
        cmp(cpu, value);
    },
    |cpu| {
        /* 0xCE */
        /* DEC nnnn */
        let address = absolute(cpu);
        dec(cpu, address);
    },
    |_| {
        /* 0xCF */
    },
    |cpu| {
        /* 0xD0 */
        /* BNE */
        let nn = i16::from(cpu.fetch_byte() as i8);
        if !cpu.registers.get_z() {
            branch(cpu, nn);
        }
    },
    |cpu| {
        /* 0xD1 */
        /* CMP (nn),Y */
        let address = indirect_y(cpu, false);
        let value = cpu.read_byte(address);
        cmp(cpu, value);
    },
    |cpu| {
        /* 0xD2 */
        /* JAM */
        jam(cpu)
    },
    |_| {
        /* 0xD3 */
    },
    |cpu| {
        /* 0xD4 */
        /* NOP nn,X */
        let address = zpg_x(cpu);
        cpu.read_byte(address);
    },
    |cpu| {
        /* 0xD5 */
        /* CMP nn,X */
        let address = zpg_x(cpu);
        let value = cpu.read_byte(address);
        cmp(cpu, value);
    },
    |cpu| {
        /* 0xD6 */
        /* DEC nn,X */
        let address = zpg_x(cpu);
        dec(cpu, address);
    },
    |_| {
        /* 0xD7 */
    },
    |cpu| {
        /* 0xD8 */
        /* CLD */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.set_d(false);
    },
    |cpu| {
        /* 0xD9 */
        /* CMP nnnn,Y */
        let address = absolute_y(cpu, false);
        let value = cpu.read_byte(address);
        cmp(cpu, value);
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
    |cpu| {
        /* 0xDD */
        /* CMP nnnn,X */
        let address = absolute_x(cpu, false);
        let value = cpu.read_byte(address);
        cmp(cpu, value);
    },
    |cpu| {
        /* 0xDE */
        /* DEC nnnn,X */
        let address = absolute_x(cpu, true);
        dec(cpu, address);
    },
    |_| {
        /* 0xDF */
    },
    |cpu| {
        /* 0xE0 */
        /* CPX #nn */
        let value = immediate(cpu);
        cpx(cpu, value);
    },
    |cpu| {
        /* 0xE1 */
        /* SBC (nn,X) */
        let address = indirect_x(cpu);
        let value = !cpu.read_byte(address);
        sbc(cpu, value);
    },
    |cpu| {
        /* 0xE2 */
        /* NOP #nn */
        let _ = immediate(cpu);
    },
    |_| {
        /* 0xE3 */
    },
    |cpu| {
        /* 0xE4 */
        /* CPX nn */
        let address = zpg(cpu);
        let value = cpu.read_byte(address);
        cpx(cpu, value);
    },
    |cpu| {
        /* 0xE5 */
        /* SBC nn */
        let address = zpg(cpu);
        let value = !cpu.read_byte(address);
        sbc(cpu, value);
    },
    |cpu| {
        /* 0xE6 */
        /* INC nn */
        let address = zpg(cpu);
        inc(cpu, address);
    },
    |_| {
        /* 0xE7 */
    },
    |cpu| {
        /* 0xE8 */
        /* INX */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.x = cpu.registers.x.wrapping_add(1);

        cpu.registers.set_z(cpu.registers.x == 0);
        cpu.registers.set_n(cpu.registers.x >> 7 == 1);
    },
    |cpu| {
        /* 0xE9 */
        /* SBC #nn */
        let value = !immediate(cpu);
        sbc(cpu, value);
    },
    |cpu| {
        /* 0xEA */
        /* NOP */
        cpu.read_byte(cpu.registers.pc);
    },
    |_| {
        /* 0xEB */
    },
    |cpu| {
        /* 0xEC */
        /* CPX nnnn */
        let address = absolute(cpu);
        let value = cpu.read_byte(address);
        cpx(cpu, value);
    },
    |cpu| {
        /* 0xED */
        /* SBC nnnn */
        let address = absolute(cpu);
        let value = !cpu.read_byte(address);
        sbc(cpu, value);
    },
    |cpu| {
        /* 0xEE */
        /* INC nnnn */
        let address = absolute(cpu);
        inc(cpu, address);
    },
    |_| {
        /* 0xEF */
    },
    |cpu| {
        /* 0xF0 */
        /* BEQ */
        let nn = i16::from(cpu.fetch_byte() as i8);
        if cpu.registers.get_z() {
            branch(cpu, nn);
        }
    },
    |cpu| {
        /* 0xF1 */
        /* SBC (nn),Y */
        let address = indirect_y(cpu, false);
        let value = !cpu.read_byte(address);
        sbc(cpu, value);
    },
    |cpu| {
        /* 0xF2 */
        /* JAM */
        jam(cpu)
    },
    |_| {
        /* 0xF3 */
    },
    |cpu| {
        /* 0xF4 */
        /* NOP nn,X */
        let address = zpg_x(cpu);
        cpu.read_byte(address);
    },
    |cpu| {
        /* 0xF5 */
        /* SBC nn,X */
        let address = zpg_x(cpu);
        let value = !cpu.read_byte(address);
        sbc(cpu, value);
    },
    |cpu| {
        /* 0xF6 */
        /* INC nn,X */
        let address = zpg_x(cpu);
        inc(cpu, address);
    },
    |_| {
        /* 0xF7 */
    },
    |cpu| {
        /* 0xF8 */
        /* SED */
        let _ = cpu.read_byte(cpu.registers.pc);
        cpu.registers.set_d(true);
    },
    |cpu| {
        /* 0xF9 */
        /* SBC nnnn,Y */
        let address = absolute_y(cpu, false);
        let value = !cpu.read_byte(address);
        sbc(cpu, value);
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
    |cpu| {
        /* 0xFD */
        /* SBC nnnn,X */
        let address = absolute_x(cpu, false);
        let value = !cpu.read_byte(address);
        sbc(cpu, value);
    },
    |cpu| {
        /* 0xFE */
        /* INC nnnn,X */
        let address = absolute_x(cpu, true);
        inc(cpu, address);
    },
    |_| {
        /* 0xFF */
    }]
};