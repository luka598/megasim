use crate::sim::naive::chip::Chip;

pub fn op_and(c: &mut Chip, rd: u8, rr: u8) -> (u8,) {
    let d = rd as usize;
    let r = rr as usize;

    let result = c.ram[d] & c.ram[r];
    c.ram[d] = result;

    let mut sreg = c.sreg_get();
    sreg.v = false;
    sreg.n = (result as i8) < 0;
    sreg.z = result == 0;
    sreg.s = sreg.n ^ sreg.v;
    c.sreg_set(&sreg);

    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_andi(c: &mut Chip, rd: u8, k: u8) -> (u8,) {
    if rd < 16 || rd > 31 {
        panic!("ANDI: Invalid Register R{}. Must be R16-R31.", rd);
    }
    let d = rd as usize;

    let result = c.ram[d] & k;
    c.ram[d] = result;

    let mut sreg = c.sreg_get();
    sreg.v = false;
    sreg.n = (result as i8) < 0;
    sreg.z = result == 0;
    sreg.s = sreg.n ^ sreg.v;
    c.sreg_set(&sreg);

    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_clr(c: &mut Chip, rd: u8) -> (u8,) {
    let d = rd as usize;
    c.ram[d] = 0;

    let mut sreg = c.sreg_get();
    sreg.s = false;
    sreg.v = false;
    sreg.n = false;
    sreg.z = true;
    c.sreg_set(&sreg);

    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_com(c: &mut Chip, rd: u8) -> (u8,) {
    let d = rd as usize;
    let val = c.ram[d];
    let result = 0xFF - val;
    c.ram[d] = result;

    let mut sreg = c.sreg_get();
    sreg.c = true;
    sreg.v = false;
    sreg.n = (result as i8) < 0;
    sreg.z = result == 0;
    sreg.s = sreg.n ^ sreg.v;
    c.sreg_set(&sreg);

    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_cpi(c: &mut Chip, rd: u8, k: u8) -> (u8,) {
    if rd < 16 || rd > 31 {
        panic!("CPI: Invalid Register R{}. Must be R16-R31.", rd);
    }
    let d = c.ram[rd as usize];
    let result = d.wrapping_sub(k);

    let d7 = (d >> 7) & 1 == 1;
    let k7 = (k >> 7) & 1 == 1;
    let r7 = (result >> 7) & 1 == 1;
    let d3 = (d >> 3) & 1 == 1;
    let k3 = (k >> 3) & 1 == 1;
    let r3 = (result >> 3) & 1 == 1;

    let mut sreg = c.sreg_get();
    sreg.h = (!d3 & k3) | (k3 & r3) | (r3 & !d3);
    sreg.v = (d7 & !k7 & !r7) | (!d7 & k7 & r7);
    sreg.n = r7;
    sreg.z = result == 0;
    sreg.c = (!d7 & k7) | (k7 & r7) | (r7 & !d7);
    sreg.s = sreg.n ^ sreg.v;
    c.sreg_set(&sreg);

    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_dec(c: &mut Chip, rd: u8) -> (u8,) {
    let d = rd as usize;
    let old_val = c.ram[d];
    let result = old_val.wrapping_sub(1);
    c.ram[d] = result;

    let mut sreg = c.sreg_get();
    sreg.v = old_val == 0x80;
    sreg.n = (result as i8) < 0;
    sreg.z = result == 0;
    sreg.s = sreg.n ^ sreg.v;
    c.sreg_set(&sreg);

    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_eor(c: &mut Chip, rd: u8, rr: u8) -> (u8,) {
    let d = rd as usize;
    let r = rr as usize;
    let result = c.ram[d] ^ c.ram[r];
    c.ram[d] = result;

    let mut sreg = c.sreg_get();
    sreg.v = false;
    sreg.n = (result as i8) < 0;
    sreg.z = result == 0;
    sreg.s = sreg.n ^ sreg.v;
    c.sreg_set(&sreg);

    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_inc(c: &mut Chip, rd: u8) -> (u8,) {
    let d = rd as usize;
    let old_val = c.ram[d];
    let result = old_val.wrapping_add(1);
    c.ram[d] = result;

    let mut sreg = c.sreg_get();
    sreg.v = old_val == 0x7F;
    sreg.n = (result as i8) < 0;
    sreg.z = result == 0;
    sreg.s = sreg.n ^ sreg.v;
    c.sreg_set(&sreg);

    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_or(c: &mut Chip, rd: u8, rr: u8) -> (u8,) {
    let d = rd as usize;
    let r = rr as usize;
    let result = c.ram[d] | c.ram[r];
    c.ram[d] = result;

    let mut sreg = c.sreg_get();
    sreg.v = false;
    sreg.n = (result as i8) < 0;
    sreg.z = result == 0;
    sreg.s = sreg.n ^ sreg.v;
    c.sreg_set(&sreg);

    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_ori(c: &mut Chip, rd: u8, k: u8) -> (u8,) {
    if rd < 16 || rd > 31 {
        panic!("ORI: Invalid Register R{}. Must be R16-R31.", rd);
    }
    let d = rd as usize;

    let result = c.ram[d] | k;
    c.ram[d] = result;

    let mut sreg = c.sreg_get();
    sreg.v = false;
    sreg.n = (result as i8) < 0;
    sreg.z = result == 0;
    sreg.s = sreg.n ^ sreg.v;
    c.sreg_set(&sreg);

    c.pc = c.pc.wrapping_add(1);
    (1,)
}
