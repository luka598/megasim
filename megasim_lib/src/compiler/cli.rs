use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write, Result};
use crate::compiler::{Op, compile};

fn save_program(
    cseg: &HashMap<u64, Op>,
    dseg: &HashMap<u64, u64>,
    path: &str,
) -> Result<()> {
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

    let mut file = File::create(path)?;
    file.write_all(output.as_bytes())?;
    Ok(())
}

pub fn cli() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: megasim <input_asm_path> <output_txt_path>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let mut source = String::new();
    let mut file = File::open(input_path).expect("Failed to open input file");
    file.read_to_string(&mut source).expect("Failed to read input file");

    let (cseg, dseg) = compile(&source);

    save_program(&cseg, &dseg, output_path).expect("Failed to save program output");
    println!("Compilation successful. Output saved to {}", output_path);
}