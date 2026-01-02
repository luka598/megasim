use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use megasim_lib::compiler::{Op, compile};

fn program_to_string(
    cseg: &HashMap<u64, Op>,
    dseg: &HashMap<u64, u64>,
) -> String {
    let mut output = String::new();

    output.push_str("--- DSEG ---\n");
    let mut dseg_addrs: Vec<_> = dseg.keys().collect();
    dseg_addrs.sort();
    for addr in dseg_addrs {
        output.push_str(&format!("{}: {:02}\n", addr, dseg[addr]));
    }

    output.push_str("\n--- CSEG ---\n");
    let mut cseg_addrs: Vec<_> = cseg.keys().collect();
    cseg_addrs.sort();
    for addr in cseg_addrs {
        let op = &cseg[addr];
        let formatted_instr = match op {
            Op::Nullary(m) => m.to_uppercase(),
            Op::Unary(m, a1) => format!("{} {}", m.to_uppercase(), a1),
            Op::Binary(m, a1, a2) => format!("{} {} {}", m.to_uppercase(), a1, a2),
            Op::Ternary(m, a1, a2, a3) => format!("{} {} {} {}", m.to_uppercase(), a1, a2, a3),
        };
        output.push_str(&format!("{}: {}\n", addr, formatted_instr));
    }

    output
}

#[wasm_bindgen(js_name = compile)]
pub fn compile_js(source: &str) -> String {
    let (cseg, dseg) = compile(source);
    program_to_string(&cseg, &dseg)
}
