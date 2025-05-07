use crate::lexer::*;
use crate::parser::NodeKind::number;


const ARM_REGISTERS: [&str; 31] = ["x0",  "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14", "x15", "x16", "x17", "x18", "x19",  "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27", "x28", "x29", "X30"];

#[derive(Copy, Clone)]
enum NodeKind {
    number,
    binary_operator(BinaryOpKind),
}

// trait NodeTrait {
//
//     fn kind(&self) -> NodeKind;
//
// }

struct Node {
    kind: NodeKind,
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(kind: NodeKind, val: i64, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self { kind, val, left, right }
    }

    fn new_number(val: i64) -> Self {
        Self { kind: NodeKind::number, val, left: None, right: None }
    }

}

struct Parser {
    tokens: Vec<Token>,
    token_index: Index,
}

impl Parser {

    fn current_token_ref(&self) -> &Token {
        &self.tokens[self.token_index]
    }

    fn consume_token(&mut self) {
        self.token_index += 1;
    }

    fn number(&mut self) -> Node {
        let token = &self.tokens[self.token_index];
        match token.kind {
            TokenKind::Literal(LiteralKind::IntNumber(val)) => {
                self.consume_token();
                Node::new_number(val)
            },
            _ => {
                panic!("unexpected token: {:?}", token);
            }
        }
    }

    fn expression(&mut self) -> Node {
        let mut root: Node = self.number();

        loop {
            let token = self.current_token_ref();
            match token.kind {
                TokenKind::BinaryOp(bin_op) => {
                    // consume bin op
                    self.consume_token();
                    root = Node::new(NodeKind::binary_operator(bin_op), bin_op as i64, Some(Box::new(root)), Some(Box::new(self.number())))
                },
                _ => {
                    break
                }
            }
        }
        let last_token = self.current_token_ref();
        if last_token.kind != TokenKind::EOF {
            panic!("stray token: {:?}", last_token);
        }
        root
    }

    fn code_gen(node: Node) -> String { // TODO: String

        "".to_string()
    }

}