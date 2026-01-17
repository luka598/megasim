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
        op_sbis, op_sbrc, op_sbrs,
    },
    data_transfer::{op_in, op_ldi, op_mov, op_out, op_pop, op_push},
};

use core::time;
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
    pub clock_freq: u64,

    // hack
    // pub flash: [u16; 8192],
    pub program: HashMap<u16, Op>,

    // also hack
    prev_int0: bool,
    prev_int1: bool,
    prev_int2: bool,
}

impl Chip {
    pub const IO_OFFSET: u16 = 32;

    pub fn new() -> Self {
        Chip {
            pc: 0,
            ram: [0; 1120],
            clock_freq: 8_000_000,

            program: HashMap::new(),

            prev_int0: false,
            prev_int1: false,
            prev_int2: false,
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

        if sreg.c {
            val |= 1 << 0;
        }
        if sreg.z {
            val |= 1 << 1;
        }
        if sreg.n {
            val |= 1 << 2;
        }
        if sreg.v {
            val |= 1 << 3;
        }
        if sreg.s {
            val |= 1 << 4;
        }
        if sreg.h {
            val |= 1 << 5;
        }
        if sreg.t {
            val |= 1 << 6;
        }
        if sreg.i {
            val |= 1 << 7;
        }

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

    pub fn ram_set_byte(&mut self, idx: usize, x: u8) {
        self.ram[idx] = x;
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

    fn _tick_interupts(&mut self) {
        // GCIR = 91
        // GCIR @ 6 => int0
        // GCIR @ 7 => int1
        // GCIR @ 5 => int2
        // ---
        // MCUCR = 85
        // ISC01(1), ISC00(0) | ISC10(2), ISC11(3)
        // 0, 0 => low
        // 0, 1 => delta
        // 1, 0 => falling edge
        // 1, 1 => rising edge
        // ISC2
        // 0 => falling edge
        // 1 => rising edge
        // ---
        // int0_signal = ??? @ ???
        // int1_signal = ??? @ ???
        // int2_signal = ??? @ ???
        // ---
        // PIND = 48
        // PINB = 54
        if !self.sreg_get().i {
            return;
        }

        pub fn set_int(c: &mut Chip, new_pc: u16) {
            c.ram[c.sp_get() as usize] = (c.pc & 0xFF) as u8;
            c.sp_add(-1);
            c.ram[c.sp_get() as usize] = (c.pc >> 8) as u8;
            c.sp_add(-1);

            let mut sreg = c.sreg_get();
            sreg.i = false;
            c.sreg_set(&sreg);

            c.pc = new_pc;
        }

        let pind = self.ram[48];
        let pinb = self.ram[54];
        let mcucr = self.ram[85];
        let gcir = self.ram[91];

        let int0_en = ((gcir >> 6) & 1) != 0;
        let int1_en = ((gcir >> 7) & 1) != 0;
        let int2_en = ((gcir >> 5) & 1) != 0;

        let isc0 = ((mcucr >> 0) & 0b11) as u8;
        let isc1 = ((mcucr >> 2) & 0b11) as u8;
        let isc2 = ((mcucr >> 6) & 0b01) as u8;

        let int0 = ((pind >> 2) & 1) != 0;
        let int1 = ((pind >> 3) & 1) != 0;
        let int2 = ((pinb >> 2) & 1) != 0;

        let int0_active = int0_en
            && match isc0 {
                0b00 => int0 == false,
                0b01 => int0 != self.prev_int0,
                0b10 => self.prev_int0 && !int0,
                0b11 => !self.prev_int0 && int0,
                _ => false,
            };
        let int1_active = int1_en
            && match isc1 {
                0b00 => int1 == false,
                0b01 => int1 != self.prev_int1,
                0b10 => self.prev_int1 && !int1,
                0b11 => !self.prev_int1 && int1,
                _ => false,
            };
        let int2_active = int2_en
            && match isc2 {
                0 => self.prev_int2 && !int2,
                1 => !self.prev_int2 && int2,
                _ => false,
            };

        if int0_active {
            set_int(self, 2);
        } else if int1_active {
            set_int(self, 4);
        } else if int2_active {
            set_int(self, 6);
        }

        self.prev_int0 = int0;
        self.prev_int1 = int1;
        self.prev_int2 = int2;
    }

    fn _tick_timers(&mut self, time_delta: u64) {}

    pub fn step(&mut self, time_delta: Option<u64>) -> bool {
        self._tick_interupts();
        self._tick_timers(time_delta.unwrap_or(1_000_000_000 / self.clock_freq));

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
                "sbrs" => op_sbrs(self, *arg1 as u8, *arg2 as u8),
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
