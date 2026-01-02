use std::vec;

//
// Tokenizer
//

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Dot,
    Comma,
    Colon,
    Semicolon,

    Equals,
    LeftParen,
    RightParen,
    Less,
    Greater,

    Space,
    Tab,
    EndOfLine,

    String(String),

    None,
}

impl Token {
    pub fn is(&self, other: &Token) -> bool {
        match self {
            Token::Dot => matches!(other, Token::Dot),
            Token::Comma => matches!(other, Token::Comma),
            Token::Colon => matches!(other, Token::Colon),
            Token::Semicolon => matches!(other, Token::Semicolon),

            Token::Equals => matches!(other, Token::Equals),
            Token::LeftParen => matches!(other, Token::LeftParen),
            Token::RightParen => matches!(other, Token::RightParen),
            Token::Less => matches!(other, Token::Less),
            Token::Greater => matches!(other, Token::Greater),

            Token::Space => matches!(other, Token::Space),
            Token::Tab => matches!(other, Token::Tab),
            Token::EndOfLine => matches!(other, Token::EndOfLine),

            Token::String(_) => matches!(other, Token::String(_)),
            Token::None => false,
        }
    }

    pub fn from_string(&self) -> String {
        match self {
            Token::String(x) => x.to_string(),
            _ => panic!("Current token is not a string!"),
        }
    }
}

pub fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let text = text.to_lowercase();
    let text = text.replace("\r", "");
    let mut current_string: Vec<char> = vec![];

    for c in text.chars() {
        let tok = match c {
            '.' => Token::Dot,
            ',' => Token::Comma,
            ':' => Token::Colon,
            ';' => Token::Semicolon,

            '=' => Token::Equals,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '<' => Token::Less,
            '>' => Token::Greater,

            // '|' => Token::Pipe,
            // '&' => Token::Ampersand,
            // '~' => Token::Tilde,

            ' ' => Token::Space,
            '\t' => Token::Tab,
            '\n' => Token::EndOfLine,
            _ => {
                current_string.push(c);
                Token::None
            }
        };

        if tok != Token::None {
            if !current_string.is_empty() {
                tokens.push(Token::String(current_string.iter().collect()));
                current_string.clear();
            }
            tokens.push(tok);
        }
    }

    if !current_string.is_empty() {
        tokens.push(Token::String(current_string.iter().collect()));
    }

    tokens.push(Token::EndOfLine);

    tokens
}

pub fn detokenize(tokens: &[Token]) -> String {
    let mut out = String::new();

    for t in tokens {
        match t {
            Token::String(s) => out.push_str(s),
            Token::Dot => out.push('.'),
            Token::Comma => out.push(','),
            Token::Colon => out.push(':'),
            Token::Semicolon => out.push(';'),

            Token::Equals => out.push('='),
            Token::LeftParen => out.push('('),
            Token::RightParen => out.push(')'),
            Token::Less => out.push('<'),
            Token::Greater => out.push('>'),

            Token::Space => out.push(' '),
            Token::Tab => out.push('\t'),
            Token::EndOfLine => out.push('\n'),

            Token::None => {}
        }
        if !matches!(t, Token::EndOfLine | Token::None) {
            out.push('|');
        }
    }

    out
}

//
// Stream
//

pub struct Stream<T> {
    pub data: Vec<T>,
    pub pos: usize,
}

impl<T> Stream<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data, pos: 0 }
    }

    pub fn current(&self) -> &T {
        &self.data[self.pos]
    }

    pub fn peek(&self, n: usize) -> Option<&T> {
        self.data.get(self.pos + n)
    }

    pub fn end(&self) -> bool {
        self.pos >= self.data.len()
    }

    pub fn advance(&mut self) {
        if !self.end() {
            self.pos += 1;
        }
    }
}

