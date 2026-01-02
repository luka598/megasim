use megasim_lib::compiler::Op;
use std::{collections::HashMap, env, fs::File, io::Read};

fn stringify_program(cseg: &HashMap<u64, Op>, dseg: &HashMap<u64, u64>) -> Result<String, ()> {
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

    Ok(output)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: megasim <input_asm_path>");
        std::process::exit(1);
    }

    let input_path = &args[1];

    let mut source = String::new();
    let mut file = File::open(input_path).expect("Failed to open input file");
    file.read_to_string(&mut source)
        .expect("Failed to read input file");

    let (cseg, dseg) = megasim_lib::compiler::compile(&source);
    println!("Compiled!");
    println!("{}", stringify_program(&cseg, &dseg).unwrap());

    let mut chip = megasim_lib::sim::naive::chip::Chip::new();
    chip.apply_cseg(&cseg).unwrap();
    chip.apply_dseg(&dseg).unwrap();

    for _ in 0..10_000 {
        println!("PC={} | PORTA={:?}", chip.pc, chip.ram[59]);
        // println!("{:?}", chip);
        if !chip.step() {
            break;
        }
    }
}
