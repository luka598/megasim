use wasm_bindgen::prelude::*;
use js_sys::{Object, Reflect, Uint8Array};
use std::collections::HashMap;

use megasim_lib::{
    compiler::{Op, compile},
    sim::naive::chip::Chip,
};

fn program_to_string(
    cseg: &HashMap<u64, Op>,
    dseg: &HashMap<u64, u64>,
) -> String {
    let mut out = String::new();

    out.push_str("--- DSEG ---\n");
    let mut daddrs: Vec<_> = dseg.keys().cloned().collect();
    daddrs.sort();
    for a in daddrs {
        out.push_str(&format!("{}: {:02}\n", a, dseg[&a]));
    }

    out.push_str("\n--- CSEG ---\n");
    let mut caddrs: Vec<_> = cseg.keys().cloned().collect();
    caddrs.sort();
    for a in caddrs {
        let op = &cseg[&a];
        let s = match op {
            Op::Nullary(m) => m.to_uppercase(),
            Op::Unary(m, a1) => format!("{} {}", m.to_uppercase(), a1),
            Op::Binary(m, a1, a2) => format!("{} {} {}", m.to_uppercase(), a1, a2),
            Op::Ternary(m, a1, a2, a3) =>
                format!("{} {} {} {}", m.to_uppercase(), a1, a2, a3),
        };
        out.push_str(&format!("{}: {}\n", a, s));
    }

    out
}

#[wasm_bindgen]
pub struct Simulator {
    chip: Chip,
    program_str: String,
}

#[wasm_bindgen]
impl Simulator {
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str) -> Simulator {
        let (cseg, dseg) = compile(source);
        let program_str = program_to_string(&cseg, &dseg);

        let mut chip = Chip::new();
        chip.apply_cseg(&cseg).unwrap();
        chip.apply_dseg(&dseg).unwrap();

        Simulator { chip, program_str }
    }

    pub fn program_str(&self) -> String {
        self.program_str.clone()
    }

    pub fn step(&mut self) -> bool {
        self.chip.step()
    }

    pub fn state(&self) -> JsValue {
        let obj = Object::new();

        Reflect::set(&obj, &"pc".into(), &(self.chip.pc as f64).into()).unwrap();
        Reflect::set(&obj, &"clock_freq".into(), &(self.chip.clock_freq as f64).into()).unwrap();
        let ram = Uint8Array::from(self.chip.ram.as_ref());
        Reflect::set(&obj, &"ram".into(), &ram.into()).unwrap();

        obj.into()
    }
}
