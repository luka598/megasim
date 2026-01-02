use std::collections::HashMap;
use crate::compiler::parser::{Directive, Expression, Function, Operator, Statement};

#[derive(Debug)]
pub enum Op {
    Nullary(String),
    Unary(String, i64),
    Binary(String, i64, i64),
    Ternary(String, i64, i64, i64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Segment {
    Cseg,
    Dseg,
    Eseg,
}

fn get_instruction_width(mnemonic: &str) -> u64 {
    match mnemonic {
        "jmp" | "call" | "lds" | "sts" => 4,
        _ => 2,
    }
}

pub fn codegen(ast: &[Statement]) -> (HashMap<u64, Op>, HashMap<u64, u64>) {
    let mut cseg: HashMap<u64, Op> = HashMap::new();
    let mut dseg: HashMap<u64, u64> = HashMap::new();
    let mut symbols: HashMap<String, i64> = crate::compiler::atmega16a::gen_symbols();

    fn eval(expr: &Expression, syms: &HashMap<String, i64>) -> i64 {
        match expr {
            Expression::Integer(n) => *n,
            Expression::Identifier(s) => *syms.get(s).expect(&format!("Undefined symbol: {}", s)),
            Expression::FunctionCall(f, arg) => {
                let v = eval(arg, syms);
                match f {
                    Function::High => (v >> 8) & 0xFF,
                    Function::Low => v & 0xFF,
                }
            }
            Expression::BinaryOp(Operator::ShiftLeft, l, r) => eval(l, syms) << eval(r, syms),
        }
    }

    let mut c_pc: u64 = 0;
    let mut d_pc: u64 = 0;
    let mut e_pc: u64 = 0;
    let mut current_seg = Segment::Cseg;

    for s in ast {
        match s {
            Statement::Directive(Directive::Cseg) => current_seg = Segment::Cseg,
            Statement::Directive(Directive::Dseg) => current_seg = Segment::Dseg,
            Statement::Directive(Directive::Eseg) => current_seg = Segment::Eseg,
            Statement::Directive(Directive::Org(expr)) => {
                let val = eval(expr, &symbols) as u64;
                match current_seg {
                    Segment::Cseg => c_pc = val * 2,
                    Segment::Dseg => d_pc = val,
                    Segment::Eseg => e_pc = val,
                }
            }
            Statement::Directive(Directive::Equ(name, expr)) => {
                let val = eval(expr, &symbols);
                symbols.insert(name.clone(), val);
            }
            Statement::Directive(Directive::Def(name, reg)) => {
                let val = *symbols.get(reg).unwrap_or(&0);
                symbols.insert(name.clone(), val);
            }
            Statement::Label(name) => {
                let addr = match current_seg {
                    Segment::Cseg => c_pc,
                    Segment::Dseg => d_pc,
                    Segment::Eseg => e_pc,
                };
                symbols.insert(name.clone(), addr as i64);
            }
            Statement::Instruction(mnemonic, _) => {
                if current_seg != Segment::Cseg {
                    panic!(
                        "Cannot place instruction {} in data/eeprom segment",
                        mnemonic
                    );
                }
                c_pc += get_instruction_width(mnemonic);
            }
        }
    }

    let mut c_pc: u64 = 0;
    let mut current_seg = Segment::Cseg;

    for s in ast {
        match s {
            Statement::Directive(Directive::Cseg) => current_seg = Segment::Cseg,
            Statement::Directive(Directive::Dseg) => current_seg = Segment::Dseg,
            Statement::Directive(Directive::Eseg) => current_seg = Segment::Eseg,
            Statement::Directive(Directive::Org(expr)) => {
                let val = eval(&expr, &symbols) as u64;
                match current_seg {
                    Segment::Cseg => c_pc = val * 2,
                    Segment::Dseg => d_pc = val,
                    Segment::Eseg => e_pc = val,
                }
            }
            Statement::Instruction(mnemonic, operands) => {
                if current_seg == Segment::Cseg {
                    let vals: Vec<i64> = operands.iter().map(|o| eval(o, &symbols)).collect();

                    let op = match vals.len() {
                        0 => Op::Nullary(mnemonic.clone()),
                        1 => Op::Unary(mnemonic.clone(), vals[0]),
                        2 => Op::Binary(mnemonic.clone(), vals[0], vals[1]),
                        3 => Op::Ternary(mnemonic.clone(), vals[0], vals[1], vals[2]),
                        _ => panic!("unknown arity"),
                    };

                    cseg.insert(c_pc, op);
                    c_pc += get_instruction_width(&mnemonic);
                }
            }
            _ => {}
        }
    }

    (cseg, dseg)
}