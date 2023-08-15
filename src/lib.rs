#[cfg(test)]
mod tests;

#[macro_use]
extern crate lazy_static;
use anyhow::Result;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenType {
    // Single-char tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // one or two char tokens.
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,
    // literals.
    IDENTIFIER,
    STRING,
    NUMBER,
    // keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug)]
enum MyError {
    ParseError(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MyError::ParseError(ref err) => write!(f, "Parsing error occurred {:?}", err),
        }
    }
}

impl std::error::Error for MyError {}

lazy_static! {
    static ref MAP: HashMap<&'static str, TokenType> = HashMap::from([
        ("and", TokenType::AND),
        ("class", TokenType::CLASS),
        ("else", TokenType::ELSE),
        ("false", TokenType::FALSE),
        ("for", TokenType::FOR),
        ("fun", TokenType::FUN),
        ("if", TokenType::IF),
        ("nil", TokenType::NIL),
        ("or", TokenType::OR),
        ("print", TokenType::PRINT),
        ("return", TokenType::RETURN),
        ("super", TokenType::SUPER),
        ("this", TokenType::THIS),
        ("true", TokenType::TRUE),
        ("var", TokenType::VAR),
        ("while", TokenType::WHILE),
    ]);
}

fn error(line: usize, msg: &str) {
    report(line, "", msg);
}

fn report(line: usize, info: &str, msg: &str) {
    println!("[line {}] Error {}: {}", line, info, msg);
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>, // TODO
    line: usize,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

pub struct Scanner {
    source: String,
    tokens: RefCell<Vec<Token>>,
    start: RefCell<usize>,
    current: RefCell<usize>,
    line: RefCell<usize>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new().into(),
            start: 0.into(),
            current: 0.into(),
            line: 1.into(),
        }
    }

    pub fn scan_tokens(&self) -> Result<Vec<Token>> {
        while !self.is_end() {
            *self.start.borrow_mut() = *self.current.borrow();
            self.scan_token()?;
        }

        self.tokens.borrow_mut().push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line: *self.line.borrow(),
        }); //todo
        Ok(self.tokens.borrow().clone())
    }

    fn scan_token(&self) -> Result<()> {
        let c: char = self.advance();
        use TokenType::*;
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),
            '!' => {
                let token = if self.is_match('=') { BangEqual } else { BANG };
                self.add_token(token);
            }
            '=' => {
                let token = if self.is_match('=') {
                    EqualEqual
                } else {
                    EQUAL
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.is_match('=') { LessEqual } else { LESS };
                self.add_token(token);
            }
            '>' => {
                let token = if self.is_match('=') {
                    GreaterEqual
                } else {
                    GREATER
                };
                self.add_token(token);
            }
            '/' => {
                if self.is_match('/') {
                    // comments
                    let iter = self.source.chars();
                    for c in iter {
                        if c == '\n' || self.is_end() {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    // slash
                    self.add_token(SLASH);
                }
            }
            ' ' | '\r' | '\t' => {
                // ignore
            }
            '\n' => *self.line.borrow_mut() += 1,
            '"' => self.deal_string(),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.deal_number(),

            _ => {
                if Self::is_alpha_underline(self.peek(0)) {
                    self.deal_identifier();
                }
            }
        }
        Ok(())
    }

    fn deal_string(&self) {
        while self.peek(0) != '"' && !self.is_end() {
            if self.peek(0) == '\n' {
                *self.line.borrow_mut() += 1;
            }
            self.advance();
        }

        if self.is_end() {
            todo!();
        }

        // The '"'
        self.advance();

        // trim quotes.
        let value = &self.source[*self.start.borrow() + 1..*self.current.borrow() - 1];

        self.add_literal_token(TokenType::STRING, value);
    }

    fn deal_number(&self) {
        while Self::is_digit(self.peek(0)) {
            self.advance();
        }
        // fractional part.
        if self.peek(0) == '.' && Self::is_digit(self.peek(1)) {
            // consume the "."
            self.advance();
            while Self::is_digit(self.peek(0)) {
                self.advance();
            }
        }

        let value = self.get_current_value();
        self.add_literal_token(TokenType::NUMBER, value)
    }

    fn deal_identifier(&self) {
        while Self::is_alpha_underline_num(self.peek(0)) {
            self.advance();
        }
        let text = self.get_current_value();
        let value_type = match MAP.get(text) {
            Some(t) => *t,
            None => TokenType::IDENTIFIER,
        };

        self.add_token(value_type)
    }

    fn get_current_value(&self) -> &str {
        &self.source[*self.start.borrow()..*self.current.borrow()]
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha_underline(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_underline_num(c: char) -> bool {
        Self::is_alpha_underline(c) || Self::is_digit(c)
    }

    fn is_match(&self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }

        let current_char = self.source.chars().nth(*self.current.borrow()).unwrap();
        if current_char != expected {
            return false;
        }

        // ok
        *self.current.borrow_mut() += 1; // maybe TODO. not update current??
        return true;
    }

    fn peek(&self, offset: usize) -> char {
        let nth = *self.current.borrow() + offset;
        if self.is_end() || nth >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(nth).unwrap();
    }

    fn add_literal_token(&self, token_type: TokenType, literal: &str) {
        let text = self.get_current_value();
        self.tokens.borrow_mut().push(Token {
            token_type,
            lexeme: text.to_string(),
            literal: Some(literal.to_string()), // TODO
            line: *self.line.borrow(),
        })
    }

    fn add_token(&self, token_type: TokenType) {
        let text = self.get_current_value();
        self.tokens.borrow_mut().push(Token {
            token_type,
            lexeme: text.to_string(),
            literal: None,
            line: *self.line.borrow(),
        })
    }

    fn advance(&self) -> char {
        let current_char = self.source.chars().nth(*self.current.borrow()).unwrap();
        *self.current.borrow_mut() += 1;
        current_char
    }

    fn is_end(&self) -> bool {
        *self.current.borrow() >= self.source.len()
    }
}

pub trait Expr: std::fmt::Debug {}

#[derive(Debug)]
struct Binary {
    left: Box<dyn Expr>,
    op: Token,
    right: Box<dyn Expr>,
}

#[derive(Debug)]
struct Literal {
    text: Option<String>,
}

#[derive(Debug)]
struct Unary {
    op: Token,
    right: Box<dyn Expr>,
}

#[derive(Debug)]
struct Grouping {
    expr: Box<dyn Expr>,
}
impl Expr for Binary {}
impl Expr for Unary {}
impl Expr for Grouping {}
impl Expr for Literal {}

/*
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
 */
pub struct Parser {
    pub tokens: Vec<Token>,
    current: RefCell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into(),
            current: 0.into(),
        }
    }

    // TODO, catch error.
    pub fn parse(&self) -> Box<dyn Expr> {
        self.expression()
    }

    fn expression(&self) -> Box<dyn Expr> {
        self.equality()
    }

    fn equality(&self) -> Box<dyn Expr> {
        self.binary_builder(
            &[TokenType::BangEqual, TokenType::EqualEqual],
            Self::comparsion,
        )
    }

    fn comparsion(&self) -> Box<dyn Expr> {
        self.binary_builder(
            &[
                TokenType::GREATER,
                TokenType::GreaterEqual,
                TokenType::LESS,
                TokenType::LessEqual,
            ],
            Self::term,
        )
    }

    fn term(&self) -> Box<dyn Expr> {
        self.binary_builder(&[TokenType::MINUS, TokenType::PLUS], Self::factor)
    }

    fn factor(&self) -> Box<dyn Expr> {
        self.binary_builder(&[TokenType::SLASH, TokenType::STAR], Self::unary)
    }

    fn unary(&self) -> Box<dyn Expr> {
        if self.is_match(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Box::new(Unary {
                op: operator.clone(),
                right,
            });
        }

        self.primary()
    }

    fn primary(&self) -> Box<dyn Expr> {
        let operator = self.peek(0);
        self.advance();
        match operator.token_type {
            TokenType::FALSE => Box::new(Literal {
                text: Some("false".into()),
            }),
            TokenType::TRUE => Box::new(Literal {
                text: Some("true".into()),
            }),
            TokenType::NIL => Box::new(Literal {
                text: Some("null".into()),
            }),
            TokenType::NUMBER | TokenType::STRING => Box::new(Literal {
                text: self.previous().literal.clone(),
            }),
            TokenType::LeftParen => {
                let expr = self.expression();
                if let Err(e) = self.consume(TokenType::RightParen, "Expect ')' after expression.")
                {
                    self.error(self.peek(0), &e.to_string());
                    panic!();
                }
                Box::new(Grouping { expr })
            }
            _ => {
                self.error(self.peek(0), "Expected expression.");
                panic!();
            }
        }
    }

    fn consume(&self, t: TokenType, msg: &str) -> Result<()> {
        if self.check(&t) {
            self.advance();
            return Ok(());
        }

        Err(MyError::ParseError(msg.into()).into())
    }

    fn binary_builder<F>(&self, t: &[TokenType], op_method: F) -> Box<dyn Expr>
    where
        F: Fn(&Self) -> Box<dyn Expr>,
    {
        let mut expr = op_method(self);
        while self.is_match(t) {
            let operator = self.previous();
            let right = op_method(self);
            expr = Box::new(Binary {
                left: expr,
                op: operator.clone(),
                right,
            })
        }

        expr
    }

    fn previous(&self) -> &Token {
        //TODO ugly
        let nth = *self.current.borrow() - 1;
        self.tokens.get(nth).unwrap()
    }

    fn is_match(&self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn advance(&self) -> &Token {
        if !self.is_end() {
            *self.current.borrow_mut() += 1;
        }

        self.tokens.get(*self.current.borrow()).unwrap()
    }

    fn check(&self, t: &TokenType) -> bool {
        if self.is_end() {
            return false;
        }

        return self.peek(0).token_type == *t;
    }

    fn is_end(&self) -> bool {
        *self.current.borrow() >= self.tokens.len()
    }

    fn peek(&self, offset: usize) -> &Token {
        let nth = *self.current.borrow() + offset;
        self.tokens.get(nth).unwrap()
    }

    fn error(&self, t: &Token, msg: &str) {
        match t.token_type {
            TokenType::EOF => self.report(t.line, "at end", msg),
            _ => self.report(t.line, &format!("{}{}", " at ", t.lexeme), msg),
        }
    }

    fn report(&self, line: usize, info: &str, msg: &str) {
        println!("[line {}] Error {}: {}", line, info, msg);
    }

    fn synchronize(&self) {
        self.advance();

        while !self.is_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.peek(0).token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => {
                    // do nothing
                }
            }

            self.advance();
        }
    }
}
