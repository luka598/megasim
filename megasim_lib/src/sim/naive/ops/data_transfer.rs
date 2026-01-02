use crate::sim::naive::chip::Chip;

pub fn op_in(c: &mut Chip, rd: u8, a: u8) -> (u8,) {
    if a > 63 {
        panic!("IN: Invalid I/O Address {}. Must be 0-63.", a);
    }

    let val = c.ram[(Chip::IO_OFFSET + a as u16) as usize];
    c.ram[rd as usize] = val;
    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_ldi(c: &mut Chip, rd: u8, k: u8) -> (u8,) {
    if rd < 16 || rd > 31 {
        panic!("LDI: Invalid Register R{}. Must be R16-R31.", rd);
    }

    c.ram[rd as usize] = k;
    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_mov(c: &mut Chip, rd: u8, rr: u8) -> (u8,) {
    c.ram[rd as usize] = c.ram[rr as usize];
    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_out(c: &mut Chip, a: u8, rr: u8) -> (u8,) {
    if a > 63 {
        panic!("OUT: Invalid I/O Address {}. Must be 0-63.", a);
    }

    let val = c.ram[rr as usize];
    c.ram[(Chip::IO_OFFSET + a as u16) as usize] = val;
    c.pc = c.pc.wrapping_add(1);
    (1,)
}

pub fn op_pop(c: &mut Chip, rd: u8) -> (u8,) {
    c.sp_add(1);
    let val = c.ram[c.sp_get() as usize];
    c.ram[rd as usize] = val;
    c.pc = c.pc.wrapping_add(1);
    (2,)
}

pub fn op_push(c: &mut Chip, rr: u8) -> (u8,) {
    let val = c.ram[rr as usize];
    c.ram[c.sp_get() as usize] = val;
    c.sp_add(-1);
    c.pc = c.pc.wrapping_add(1);
    (2,)
}
