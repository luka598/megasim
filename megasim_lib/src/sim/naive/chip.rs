use crate::compiler::Op;
use crate::sim::naive::ops::{
    arithmetic_and_logic::{
        op_and, op_andi, op_clr, op_com, op_cpi, op_dec, op_eor, op_inc, op_or, op_ori,
    },
    bit_and_bittest::{
        op_cbi, op_clc, op_clt, op_lsl, op_rol, op_ror, op_sbi, op_sec, op_sei, op_set,
    },
    branch::{
        op_brcc, op_breq, op_brne, op_brtc, op_brts, op_cpse, op_rcall, op_ret, op_reti, op_rjmp,
        op_sbis, op_sbrc,
    },
    data_transfer::{op_in, op_ldi, op_mov, op_out, op_pop, op_push},
};

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Default)]
pub struct Sreg {
    pub c: bool,
    pub z: bool,
    pub n: bool,
    pub v: bool,
    pub s: bool,
    pub h: bool,
    pub t: bool,
    pub i: bool,
}

#[derive(Debug)]
pub struct Chip {
    pub pc: u16,
    pub ram: [u8; 1120], // 0-31: R0-R31 | 32-95: I/O Reg | 96-1119: SRAM

    // hack
    // pub flash: [u16; 8192],
    pub program: HashMap<u16, Op>,

    pub clock_freq: u64,
}

impl Chip {
    pub const IO_OFFSET: u16 = 32;

    pub fn new() -> Self {
        Chip {
            pc: 0,
            ram: [0; 1120],
            program: HashMap::new(),
            clock_freq: 8_000_000,
        }
    }

    pub fn sreg_get(&self) -> Sreg {
        let val = *self.ram.get(95).unwrap();
        
        Sreg { 
            c: (val & (1 << 0)) != 0,
            z: (val & (1 << 1)) != 0,
            n: (val & (1 << 2)) != 0,
            v: (val & (1 << 3)) != 0,
            s: (val & (1 << 4)) != 0,
            h: (val & (1 << 5)) != 0,
            t: (val & (1 << 6)) != 0,
            i: (val & (1 << 7)) != 0,
        }
    }

    pub fn sreg_set(&mut self, sreg: &Sreg) {
        let mut val = 0u8;

        if sreg.c { val |= 1 << 0; }
        if sreg.z { val |= 1 << 1; }
        if sreg.n { val |= 1 << 2; }
        if sreg.v { val |= 1 << 3; }
        if sreg.s { val |= 1 << 4; }
        if sreg.h { val |= 1 << 5; }
        if sreg.t { val |= 1 << 6; }
        if sreg.i { val |= 1 << 7; }

        *self.ram.get_mut(95).unwrap() = val;
    }

    pub fn sp_get(&self) -> u16 {
        let sph = *self.ram.get(94).unwrap() as u16;
        let spl = *self.ram.get(93).unwrap() as u16;

        (sph << 8) | spl
    }

    pub fn sp_set(&mut self, sp: u16) {
        *self.ram.get_mut(93).unwrap() = (sp & 0xFF) as u8;
        *self.ram.get_mut(94).unwrap() = ((sp >> 8) & 0xFF) as u8;
    }

    pub fn sp_add(&mut self, x: i16) {
        if x >= 0 {
            self.sp_set(self.sp_get().wrapping_add(x as u16));
        } else {
            self.sp_set(self.sp_get().wrapping_sub(x.abs() as u16));
        }
    }

    pub fn get_instr_size(&self, _addr: u16) -> u16 {
        1
    }

    pub fn apply_dseg(&mut self, dseg: &HashMap<u64, u64>) -> Result<(), &str> {
        for (addr, value) in dseg {
            if *addr >= self.ram.len() as u64 {
                return Err("DSEG overflow");
            }

            self.ram[*addr as usize] = *value as u8;
        }

        Ok(())
    }

    pub fn apply_cseg(&mut self, cseg: &HashMap<u64, Op>) -> Result<(), &str> {
        for (addr, op) in cseg {
            self.program.insert(*addr as u16, op.clone());
        }

        Ok(())
    }

    pub fn step(&mut self) -> bool {
        let op = match self.program.get(&self.pc) {
            Some(x) => x,
            None => {
                return false;
            }
        };

        let (_cycles,) = match op {
            Op::Nullary(mnemonic) => match mnemonic.as_str() {
                // Branch / Control
                "ret" => op_ret(self),
                "reti" => op_reti(self),
                // Bit / Bittest
                "clc" => op_clc(self),
                "clt" => op_clt(self),
                "sec" => op_sec(self),
                "sei" => op_sei(self),
                "set" => op_set(self),
                _ => panic!("Unknown instruction: {}", mnemonic),
            },

            Op::Unary(mnemonic, arg1) => match mnemonic.as_str() {
                // Arithmetic and Logic
                "clr" => op_clr(self, *arg1 as u8),
                "com" => op_com(self, *arg1 as u8),
                "dec" => op_dec(self, *arg1 as u8),
                "inc" => op_inc(self, *arg1 as u8),
                // Branch
                "rjmp" => op_rjmp(self, *arg1 as i16),
                "rcall" => op_rcall(self, *arg1 as i16),
                // Conditional Branches
                "brcc" => op_brcc(self, *arg1 as i8),
                "breq" => op_breq(self, *arg1 as i8),
                "brne" => op_brne(self, *arg1 as i8),
                "brtc" => op_brtc(self, *arg1 as i8),
                "brts" => op_brts(self, *arg1 as i8),
                // Data Transfer
                "pop" => op_pop(self, *arg1 as u8),
                "push" => op_push(self, *arg1 as u8),
                // Bit / Bittest
                "lsl" => op_lsl(self, *arg1 as u8),
                "rol" => op_rol(self, *arg1 as u8),
                "ror" => op_ror(self, *arg1 as u8),
                _ => panic!("Unknown instruction: {}", mnemonic),
            },

            Op::Binary(mnemonic, arg1, arg2) => match mnemonic.as_str() {
                // Arithmetic and Logic
                "and" => op_and(self, *arg1 as u8, *arg2 as u8),
                "andi" => op_andi(self, *arg1 as u8, *arg2 as u8),
                "cpi" => op_cpi(self, *arg1 as u8, *arg2 as u8),
                "eor" => op_eor(self, *arg1 as u8, *arg2 as u8),
                "or" => op_or(self, *arg1 as u8, *arg2 as u8),
                "ori" => op_ori(self, *arg1 as u8, *arg2 as u8),
                // Branch
                "cpse" => op_cpse(self, *arg1 as u8, *arg2 as u8),
                "sbis" => op_sbis(self, *arg1 as u8, *arg2 as u8),
                "sbrc" => op_sbrc(self, *arg1 as u8, *arg2 as u8),
                // Data Transfer
                "in" => op_in(self, *arg1 as u8, *arg2 as u8),
                "ldi" => op_ldi(self, *arg1 as u8, *arg2 as u8),
                "mov" => op_mov(self, *arg1 as u8, *arg2 as u8),
                "out" => op_out(self, *arg1 as u8, *arg2 as u8),
                // Bit / Bittest
                "cbi" => op_cbi(self, *arg1 as u8, *arg2 as u8),
                "sbi" => op_sbi(self, *arg1 as u8, *arg2 as u8),
                _ => panic!("Unknown instruction: {}", mnemonic),
            },

            Op::Ternary(_, _, _, _) => {
                panic!("Ternary not implemented");
            }
        };

        return true;
    }
}
