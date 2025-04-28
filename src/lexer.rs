use std::{f32::DIGITS, ffi::FromVecWithNulError, fs, string};

#[derive(Debug)]
#[derive(Clone, PartialEq)]
pub enum BinaryOpKind {
    Add,
    Minus,
    Multi,
    Div,
}

#[derive(Debug)]
#[derive(Clone, PartialEq)]
pub enum TokenKind {
    None, 
    IntNumber(i64),
    FloatNumber(f64),
    BinaryOp(BinaryOpKind),
    EOF,
}

#[derive(Debug)]
#[derive(Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    line: i64,
    str: String, // TODO: wangya
}

impl Token  {
    pub fn new(kind: TokenKind, line: i64, str: String ) -> Self {
        Token { kind, line, str }
    }   
}

pub struct Lexer {
    source_code: String, // TODO: wangya
    
}

impl Lexer {
    
    // read from file path
    // pub fn new(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
    //     let source = fs::read_to_string(file_path)?;

    //     Ok(Self {
    //         source_code: source
    //     })
    // }

    pub fn new(source_code: String) -> Self {
        Self {
            source_code: source_code 
        }
    }

    pub fn tokenize(self) -> Vec<Token> {
        let mut tks: Vec<Token> = Vec::new();

        
        for (i, c) in self.source_code.char_indices() {
            if let Some(digit) = c.to_digit(10) {
                let kind = TokenKind::IntNumber(digit as i64);
                let token = Token::new(kind, 1, self.source_code[i..i+1].to_string());
                tks.push(token);    
            }
            println!("token {} :{}", i, c.to_string());
        }

        tks
    }
}