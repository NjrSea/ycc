pub type Index = usize;

#[derive(Debug)]
#[derive(Copy, Clone, PartialEq)]
pub enum BinaryOpKind {
    Add,
    Minus,
    Multi,
    Div,
}

impl BinaryOpKind {

    pub fn from_char(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '-' => Self::Minus,
            '*' => Self::Multi,
            '/' => Self::Div,
            _ => panic!("Invalid binary operator: {}", c),
        }
    }

}

#[derive(Debug)]
#[derive(Copy, Clone, PartialEq)]
pub enum TokenKind {
    None, 
    BinaryOp(BinaryOpKind),
    Literal(LiteralKind),
    Space,
    EOF,
}

#[derive(Debug)]
#[derive(Copy, Clone, PartialEq)]
pub enum LiteralKind {
    IntNumber(i64),
    FloatNumber(f64),
}

#[derive(Debug)]
#[derive(Clone, PartialEq)]
pub struct Position {
   pub(crate) line: Index,
   pub(crate) column_start: Index,
   pub(crate) column_end: Index,
   pub(crate) is_eof: bool,
}

impl Position {

    fn new(line: Index, column_start: Index, column_end: Index) -> Self {
        Position {
            line: line,
            column_start: column_start,
            column_end: column_end,
            is_eof: false,
        }
    }

    fn eof() -> Position {
        Position {
            line: 0,
            column_start: 0,
            column_end: 0,
            is_eof: true,
        }
    }
}

#[derive(Debug)]
#[derive(Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Position,
    pub str: String, // TODO: wangya
}

impl Token  {
    pub fn new(kind: TokenKind, pos: Position, str: String ) -> Self {
        Token { kind, pos, str }
    }

    pub fn eof() -> Token {
        Token::new(TokenKind::EOF, Position::eof(), "".to_string())
    }
}

// enum LexerError { // TODO: handle error
//     UnexpectedCharacter{char: char},
// }

pub struct Lexer {
    pre_index: Index,
    current_index: Index,
    source_code: String, 
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
            source_code: source_code,
            pre_index: 0,
            current_index: 0,
        }
    }

    fn get_current(&self) -> Option<char> {
        self.get_char(self.current_index)
    }

    fn get_char(&self, index: Index) -> Option<char> {
        match index {
            n if n < self.source_code.chars().count() => {
                return self.source_code.chars().nth(index);
            }
            _ => {
                return None;
            }
        }
    }

    // 查看下一个字符
    fn peek(&self) -> Option<char> {
        self.get_char(self.current_index + 1)
    }

    // 移动到下一个字符
    fn next(&mut self) -> Option<char> {
        self.current_index += 1;
        self.get_current()
    }

    fn unread(&mut self) {
        self.current_index -= 1;
    }

    fn generate_token(&mut self, kind: TokenKind) -> Token {
        let range = self.pre_index..self.current_index+1;
        let str = self.source_code.get(range).unwrap();
        let pos = Position::new(1, self.pre_index, self.current_index);
        let token = Token::new(kind, pos, str.to_string());
        self.current_index += 1;
        self.pre_index = self.current_index;
        token
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tks: Vec<Token> = Vec::new();

        loop {
            let token = self.scan().unwrap(); // TODO: wangya 
            let is_eof = matches!(token.kind, TokenKind::EOF);
            let is_space = matches!(token.kind, TokenKind::Space);
            println!("token: {}", token.str);
            if !is_space {
                tks.push(token);
            }
            if is_eof {
                break
            }
        }
        tks
    }

    fn get_current_str(&self) -> String {
        self.get_str_from_range(self.pre_index, self.current_index)
    }

    fn get_str_from_range(&self, start: Index, end: Index) -> String {
        self.source_code[start..end+1].to_string()
    }

    fn scan(&mut self) -> Option<Token> {
        match self.get_current() {
            Some(ch) => {
                match ch {
                    ' ' => {
                        self.scan_space()
                    },
                    n if n == '+' || n == '-' || n == '*' || n == '/' => { // TODO: wangya
                        self.scan_op()
                    },
                    n if n.is_digit(10) => {
                        self.scan_number()
                    },
                    _ => {
                        panic!("unexpected character: {}", ch);
                    }
                }
           },
           None => {
                Some(Token::eof()) // TODO: is this right?
           }
        }
    }
    
    fn scan_op(&mut self) -> Option<Token> {
        self.scan_binary_op() // TODO: wangya ++ -- /= *=
    }

    fn scan_binary_op(&mut self) -> Option<Token> {
        let ch = self.get_current().unwrap();
        if ch == '+' || ch == '-' || ch == '*' || ch == '/' {
            let kind = TokenKind::BinaryOp(BinaryOpKind::from_char(ch));
            let token = self.generate_token(kind);
            return Some(token);
        }
        panic!("unexpected character: {}", ch);
    }

    fn scan_space(&mut self) -> Option<Token> {
        loop {
            let ch = self.get_current().unwrap(); // TODO: wangya handle unwrap

            if ch.is_whitespace() {
                let next_ch = self.next().unwrap(); // TODO:

                if !next_ch.is_whitespace() {
                    self.unread();
                    break;
                }
            } else {
                break
            }
        }
        Some(self.generate_token(TokenKind::Space))
    }
    
    fn scan_number(&mut self) -> Option<Token> {
        loop {
            let next_ch = self.peek();
            
            if let Some(next_ch) = next_ch {
                if next_ch.is_digit(10) {
                    self.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        // TODO: wangya 过于丑陋
        let token_str = self.get_current_str();
        let token_val = token_str.parse::<i64>().unwrap(); // TODO: wangya i64不准确
        let kind = TokenKind::Literal(LiteralKind::IntNumber(token_val)); // TODO: wangya IntNumber不准确
        let token = self.generate_token(kind);
        Some(token)
    }

}