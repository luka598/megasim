use crate::sim::naive::chip::Chip;

pub fn op_brcc(c: &mut Chip, k: i8) -> (u8,) {
    let sreg = c.sreg_get();
    if !sreg.c {
        c.pc = (c.pc as i32 + k as i32 + 1) as u16;
        return (2,);
    } else {
        c.pc = c.pc.wrapping_add(1);
        return (1,);
    }
}

pub fn op_breq(c: &mut Chip, k: i8) -> (u8,) {
    let sreg = c.sreg_get();
    if sreg.z {
        c.pc = (c.pc as i32 + k as i32 + 1) as u16;
        return (2,);
    } else {
        c.pc = c.pc.wrapping_add(1);
        return (1,);
    }
}

pub fn op_brne(c: &mut Chip, k: i8) -> (u8,) {
    let sreg = c.sreg_get();
    if !sreg.z {
        c.pc = (c.pc as i32 + k as i32 + 1) as u16;
        return (2,);
    } else {
        c.pc = c.pc.wrapping_add(1);
        return (1,);
    }
}

pub fn op_brtc(c: &mut Chip, k: i8) -> (u8,) {
    let sreg = c.sreg_get();
    if !sreg.t {
        c.pc = (c.pc as i32 + k as i32 + 1) as u16;
        return (2,);
    } else {
        c.pc = c.pc.wrapping_add(1);
        return (1,);
    }
}

pub fn op_brts(c: &mut Chip, k: i8) -> (u8,) {
    let sreg = c.sreg_get();
    if sreg.t {
        c.pc = (c.pc as i32 + k as i32 + 1) as u16;
        return (2,);
    } else {
        c.pc = c.pc.wrapping_add(1);
        return (1,);
    }
}

pub fn op_cpse(c: &mut Chip, rd: u8, rr: u8) -> (u8,) {
    let d = rd as usize;
    let r = rr as usize;

    if c.ram[d] == c.ram[r] {
        let next_pc = c.pc.wrapping_add(1);
        let skip_size = c.get_instr_size(next_pc);
        c.pc = next_pc.wrapping_add(skip_size);
        return if skip_size == 2 { (3,) } else { (2,) };
    } else {
        c.pc = c.pc.wrapping_add(1);
        return (1,);
    }
}

pub fn op_rcall(c: &mut Chip, k: i16) -> (u8,) {
    let ret_addr = c.pc.wrapping_add(1);

    c.ram[c.sp_get() as usize] = (ret_addr & 0xFF) as u8;
    c.sp_add(-1);

    c.ram[c.sp_get() as usize] = ((ret_addr >> 8) & 0xFF) as u8;
    c.sp_add(-1);

    c.pc = (c.pc as i32 + k as i32 + 1) as u16;
    (3,)
}

pub fn op_ret(c: &mut Chip) -> (u8,) {
    c.sp_add(1);
    let high = c.ram[c.sp_get() as usize] as u16;

    c.sp_add(1);
    let low = c.ram[c.sp_get() as usize] as u16;

    let ret_addr = (high << 8) | low;
    c.pc = ret_addr;
    (4,)
}

pub fn op_reti(c: &mut Chip) -> (u8,) {
    c.sp_add(1);
    let high = c.ram[c.sp_get() as usize] as u16;

    c.sp_add(1);
    let low = c.ram[c.sp_get() as usize] as u16;

    let ret_addr = (high << 8) | low;
    c.pc = ret_addr;

    let mut sreg = c.sreg_get();
    sreg.i = true;
    c.sreg_set(&sreg);
    (4,)
}

pub fn op_rjmp(c: &mut Chip, k: i16) -> (u8,) {
    c.pc = (c.pc as i32 + k as i32 + 1) as u16;
    (2,)
}

pub fn op_sbis(c: &mut Chip, a: u8, b: u8) -> (u8,) {
    if a > 31 {
        panic!("SBIS: Invalid I/O Address {}. Must be 0-31.", a);
    }
    if b > 7 {
        panic!("SBIS: Invalid bit {}. Must be 0-7.", b);
    }

    let io_val = c.ram[(Chip::IO_OFFSET + a as u16) as usize];

    if (io_val >> b) & 1 == 1 {
        let next_pc = c.pc.wrapping_add(1);
        let skip_size = c.get_instr_size(next_pc);
        c.pc = next_pc.wrapping_add(skip_size);
        return if skip_size == 2 { (3,) } else { (2,) };
    } else {
        c.pc = c.pc.wrapping_add(1);
        return (1,);
    }
}

pub fn op_sbrc(c: &mut Chip, rr: u8, b: u8) -> (u8,) {
    if b > 7 {
        panic!("SBRC: Invalid bit {}. Must be 0-7.", b);
    }

    let val = c.ram[rr as usize];

    if (val >> b) & 1 == 0 {
        let next_pc = c.pc.wrapping_add(1);
        let skip_size = c.get_instr_size(next_pc);
        c.pc = next_pc.wrapping_add(skip_size);
        return if skip_size == 2 { (3,) } else { (2,) };
    } else {
        c.pc = c.pc.wrapping_add(1);
        return (1,);
    }
}
