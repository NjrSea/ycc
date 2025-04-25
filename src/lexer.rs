use std::{ffi::FromVecWithNulError, fs, string};

#[derive(Clone)]
pub enum TokenKind {
    None, 
    IntNumber(i64),
    FloatNumber(f64),
    EOF,
}

#[derive(Clone)]
pub struct Token {
    kind: TokenKind,
    line: i64,
    str: String, // TODO: wangya
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
            
        }

        tks
    }
}