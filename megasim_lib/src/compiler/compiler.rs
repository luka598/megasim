use std::collections::HashMap;
use crate::compiler::codegen::Op;
use crate::compiler::lexer::tokenize;
use crate::compiler::parser::parse;
use crate::compiler::codegen::codegen;

pub fn compile(text: &str) -> (HashMap<u64, Op>, HashMap<u64, u64>) {
    codegen(&parse(&tokenize(text)))
}