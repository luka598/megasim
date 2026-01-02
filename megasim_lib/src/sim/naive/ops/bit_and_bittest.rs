use crate::sim::naive::chip::Chip;

pub fn op_cbi(c: &mut Chip, a: u8, b: u8) -> (u8,) {
    if a > 31 {
        panic!("CBI: Invalid I/O Address {}. Must be 0-31.", a);
    }
    if b > 7 {
        panic!("CBI: Invalid bit {}. Must be 0-7.", b);
    }

    let addr = (Chip::IO_OFFSET + a as u16) as usize;
    let val = c.ram[addr];
    let mask = !(1 << b);
    c.ram[addr] = val & mask;
    c.pc = c.pc.wrapping_add(1);
    (2,)
}

pub fn op_clc(c: &mut Chip) -> (u8,) {
    let mut sreg = c.sreg_get();
    sreg.c = false;
    c.sreg_set(&sreg);
    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_clt(c: &mut Chip) -> (u8,) {
    let mut sreg = c.sreg_get();
    sreg.t = false;
    c.sreg_set(&sreg);
    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_lsl(c: &mut Chip, rd: u8) -> (u8,) {
    let d = rd as usize;
    let val = c.ram[d];
    let carry = (val >> 7) & 1 == 1;
    let result = (val << 1) & 0xFF;
    c.ram[d] = result;

    let mut sreg = c.sreg_get();
    sreg.c = carry;
    sreg.h = (val >> 3) & 1 == 1;
    sreg.n = (result >> 7) & 1 == 1;
    sreg.z = result == 0;
    sreg.v = sreg.n ^ sreg.c;
    sreg.s = sreg.n ^ sreg.v;
    c.sreg_set(&sreg);

    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_rol(c: &mut Chip, rd: u8) -> (u8,) {
    let mut sreg = c.sreg_get();
    let d = rd as usize;
    let val = c.ram[d];
    let old_carry = if sreg.c { 1 } else { 0 };
    let new_carry = (val >> 7) & 1 == 1;
    let result = ((val << 1) | old_carry) & 0xFF;
    c.ram[d] = result;

    sreg.c = new_carry;
    sreg.h = (val >> 3) & 1 == 1;
    sreg.n = (result >> 7) & 1 == 1;
    sreg.z = result == 0;
    sreg.v = sreg.n ^ sreg.c;
    sreg.s = sreg.n ^ sreg.v;
    c.sreg_set(&sreg);

    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_ror(c: &mut Chip, rd: u8) -> (u8,) {
    let mut sreg = c.sreg_get();
    let d = rd as usize;
    let val = c.ram[d];
    let old_carry = if sreg.c { 1 } else { 0 };
    let new_carry = (val & 1) == 1;
    let result = (val >> 1) | (old_carry << 7);
    c.ram[d] = result;

    sreg.c = new_carry;
    sreg.n = (result >> 7) & 1 == 1;
    sreg.z = result == 0;
    sreg.v = sreg.n ^ sreg.c;
    sreg.s = sreg.n ^ sreg.v;
    c.sreg_set(&sreg);

    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_sbi(c: &mut Chip, a: u8, b: u8) -> (u8,) {
    if a > 31 {
        panic!("SBI: Invalid I/O Address {}. Must be 0-31.", a);
    }
    if b > 7 {
        panic!("SBI: Invalid bit {}. Must be 0-7.", b);
    }

    let addr = (Chip::IO_OFFSET + a as u16) as usize;
    let val = c.ram[addr];
    let mask = 1 << b;
    c.ram[addr] = val | mask;
    c.pc = c.pc.wrapping_add(1);
    (2,)
}

pub fn op_sec(c: &mut Chip) -> (u8,) {
    let mut sreg = c.sreg_get();
    sreg.c = true;
    c.sreg_set(&sreg);
    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_sei(c: &mut Chip) -> (u8,) {
    let mut sreg = c.sreg_get();
    sreg.i = true;
    c.sreg_set(&sreg);
    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_set(c: &mut Chip) -> (u8,) {
    let mut sreg = c.sreg_get();
    sreg.t = true;
    c.sreg_set(&sreg);
    c.pc = c.pc.wrapping_add(1);
    (1,)
}
