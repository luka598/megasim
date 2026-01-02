use crate::compiler::lexer::{Token, Stream};

//
// Parser types
//

#[derive(Debug)]
pub enum Statement {
    Label(String),
    Instruction(String, Vec<Expression>),
    Directive(Directive),
}

#[derive(Debug)]
pub enum Directive {
    Equ(String, Expression),
    Def(String, String),
    Org(Expression),
    Cseg,
    Dseg,
    Eseg,
}

#[derive(Debug)]
pub enum Expression {
    Integer(i64),
    Identifier(String),
    BinaryOp(Operator, Box<Expression>, Box<Expression>),
    FunctionCall(Function, Box<Expression>),
}

#[derive(Debug)]
pub enum Function {
    High,
    Low,
}

#[derive(Debug)]
pub enum Operator {
    ShiftLeft,
}

//
// Parser implementation
//

fn capture_until<'a>(tb: &'a mut Stream<Token>, tokens: &[Token]) -> &'a [Token] {
    let start = tb.pos;
    while !tb.end() && !tokens.iter().any(|t| tb.current().is(t)) {
        tb.advance();
    }
    &tb.data[start..tb.pos]
}

fn capture_only<'a>(tb: &'a mut Stream<Token>, tokens: &[Token]) -> &'a [Token] {
    let start = tb.pos;
    while !tb.end() && tokens.iter().any(|t| tb.current().is(t)) {
        tb.advance();
    }
    &tb.data[start..tb.pos]
}

//
// Expression Parser
//

fn parse_expression(tb: &mut Stream<Token>) -> Expression {
    capture_only(tb, &[Token::Space]);

    let mut expr = match tb.current() {
        Token::LeftParen => {
            tb.advance();
            let inner = parse_expression(tb);
            capture_only(tb, &[Token::Space]);
            if !tb.current().is(&Token::RightParen) {
                panic!("Expected ')' at pos {}, found {:?}", tb.pos, tb.current());
            }
            tb.advance();
            inner
        }

        Token::String(s) => {
            let val = s.clone();
            tb.advance();

            if val == "high" || val == "low" {
                let func = if val == "high" {
                    Function::High
                } else {
                    Function::Low
                };
                capture_only(tb, &[Token::Space]);
                if !tb.current().is(&Token::LeftParen) {
                    panic!("Assembler function {} expects '('", val);
                }
                tb.advance();
                let arg = parse_expression(tb);
                capture_only(tb, &[Token::Space]);
                if !tb.current().is(&Token::RightParen) {
                    panic!("Assembler function {} expects ')'", val);
                }
                tb.advance();
                Expression::FunctionCall(func, Box::new(arg))
            } else if let Ok(num) = val.parse::<i64>() {
                Expression::Integer(num)
            } else if val.starts_with("0x") {
                let num = i64::from_str_radix(&val[2..], 16).unwrap_or(0);
                Expression::Integer(num)
            } else {
                Expression::Identifier(val)
            }
        }
        _ => panic!("Unexpected token in expression: {:?}", tb.current()),
    };

    capture_only(tb, &[Token::Space]);
    if !tb.end() && tb.current().is(&Token::Less) {
        if let Some(next) = tb.peek(1) {
            if next.is(&Token::Less) {
                tb.advance();
                tb.advance();
                let right = parse_expression(tb);
                expr = Expression::BinaryOp(Operator::ShiftLeft, Box::new(expr), Box::new(right));
            }
        }
    }

    expr
}

//
// Directive Parser
//

fn parse_directive(tb: &mut Stream<Token>) -> Statement {
    tb.advance();
    let dir_name = tb.current().from_string();
    tb.advance();

    let filler = &[Token::Space, Token::Equals];

    let directive = match dir_name.as_str() {
        "equ" => {
            // .equ NAME = VALUE
            capture_only(tb, filler);
            let name = tb.current().from_string();
            tb.advance();

            capture_only(tb, filler);
            let value = parse_expression(tb);
            Directive::Equ(name, value)
        }
        "def" => {
            // .def NAME = REGISTER
            capture_only(tb, filler);
            let name = tb.current().from_string();
            tb.advance();

            capture_only(tb, filler);
            let register = tb.current().from_string();
            tb.advance();
            Directive::Def(name, register)
        }
        "org" => {
            // .org expression
            capture_only(tb, filler);
            let value = parse_expression(tb);
            Directive::Org(value)
        }
        "cseg" => Directive::Cseg,
        "dseg" => Directive::Dseg,
        "eseg" => Directive::Dseg,
        x => panic!("Unknown directive: \".{}\"", x),
    };

    Statement::Directive(directive)
}

fn parse_instruction(tb: &mut Stream<Token>) -> Statement {
    let mnemonic = tb.current().from_string();
    tb.advance();

    let mut operands = vec![];

    loop {
        capture_only(tb, &[Token::Space]);
        if tb.end() || tb.current().is(&Token::EndOfLine) || tb.current().is(&Token::Semicolon) {
            break;
        }

        operands.push(parse_expression(tb));

        capture_only(tb, &[Token::Space]);

        if !tb.end() && tb.current().is(&Token::Comma) {
            tb.advance();
        } else {
            break;
        }
    }

    Statement::Instruction(mnemonic, operands)
}

fn parse_label(tb: &mut Stream<Token>) -> Statement {
    let name = tb.current().from_string();
    tb.advance();
    tb.advance();

    Statement::Label(name)
}

fn parse_line(tb: &mut Stream<Token>) -> Vec<Statement> {
    let mut statements = vec![];

    if tb.current().is(&Token::Dot)
        && tb
            .peek(1)
            .unwrap_or(&Token::None)
            .is(&Token::String("".to_string()))
    {
        statements.push(parse_directive(tb));
    }

    if tb.current().is(&Token::String("".to_string()))
        && (tb.peek(1).unwrap_or(&Token::None).is(&Token::Space)
            || tb.peek(1).unwrap_or(&Token::None).is(&Token::EndOfLine))
    {
        statements.push(parse_instruction(tb));
    }

    if tb.current().is(&Token::String("".to_string()))
        && tb.peek(1).unwrap_or(&Token::None).is(&Token::Colon)
    {
        statements.push(parse_label(tb));
    }

    capture_until(tb, &[Token::EndOfLine]);

    statements
}

pub fn parse(tokens: &[Token]) -> Vec<Statement> {
    let mut ir: Vec<Statement> = vec![Statement::Directive(Directive::Cseg)];

    let mut tb = Stream::new(tokens.to_vec());

    while !tb.end() {
        if tb.current().is(&Token::Space) || tb.current().is(&Token::EndOfLine) {
            tb.advance();
        } else {
            for s in parse_line(&mut tb) {
                ir.push(s);
            }

            if !(tb.end() || tb.current().is(&Token::EndOfLine)) {
                panic!(
                    "Falied to parse full line! Current token: \"{:?}\"",
                    tb.current()
                )
            }
        }
    }

    ir
}