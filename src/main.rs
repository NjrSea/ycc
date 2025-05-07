// Project: YCC
// A Toy C Compiler for self-education and fun
// Reference: https://github.com/rui314/9cc
use std::env;

use lexer::{Lexer, Position, Token};
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    // Print the prologue
    println!("      .section __TEXT,__text");
    println!("      .global _main");
    println!("_main:");
    println!("      mov w0, #{}", input);
    println!("      ret");
}

#[test]
fn lexer_test_1() {
    // 0123456789
    // 5 + 20 - 4
    let lexer = Lexer::new("5 + 20 -   4".to_string());
    let tokens = lexer.tokenize();
    let token1 = Token::new(
        lexer::TokenKind::Literal(lexer::LiteralKind::IntNumber(5)),
        Position {
            line: 1,
            column_start: 0,
            column_end: 0,
            is_eof: false,
        },
        "5".to_string(),
    );
    let token1_vec = tokens.first().unwrap().clone();
    assert_eq!(token1, token1_vec);

    let token2 = Token::new(
        lexer::TokenKind::BinaryOp(lexer::BinaryOpKind::Add),
        Position {
            line: 1,
            column_start: 2,
            column_end: 2,
            is_eof: false,
        },
        "+".to_string(),
    );
    let token2_vec = tokens.get(1).unwrap().clone();
    assert_eq!(token2, token2_vec);

    let token3 = Token::new(
        lexer::TokenKind::Literal(lexer::LiteralKind::IntNumber(20)),
        Position {
            line: 1,
            column_start: 4,
            column_end: 5,
            is_eof: false,
        },
        "20".to_string(),
    );
    let token3_vec = tokens.get(2).unwrap().clone();
    assert_eq!(token3, token3_vec);

    let token4 = Token::new(
        lexer::TokenKind::BinaryOp(lexer::BinaryOpKind::Minus),
        Position {
            line: 1,
            column_start: 7,
            column_end: 7,
            is_eof: false,
        },
        "-".to_string(),
    );
    let token4_vec = tokens.get(3).unwrap().clone();
    assert_eq!(token4, token4_vec);

    let token5 = Token::new(
        lexer::TokenKind::Literal(lexer::LiteralKind::IntNumber(4)),
        Position {
            line: 1,
            column_start: 11,
            column_end: 11,
            is_eof: false,
        },
        "4".to_string(),
    );
    let token5_vec = tokens.get(4).unwrap().clone();
    assert_eq!(token5, token5_vec);

    let token6 = Token::eof();
    let token6_vec = tokens.get(5).unwrap().clone();
    assert_eq!(token6, token6_vec);
}

// #[test]
// fn compare_with_clang_output() {
//     use std::fs;
//     use std::process::Command;
//     use std::io::Write;
//     use std::io::stderr;

//     let example_paths = match fs::read_dir("examples") {
//         Ok(paths) => paths,
//         Err(e) =>  panic!("Error reading examples directory: {}", e)
//     };

//     for path in example_paths {
//         let name = path.unwrap().path().to_str().unwrap().to_string();

//         writeln!(&mut stderr(), "comparing {} ...", name).unwrap();
//     }
// }
