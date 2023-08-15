use crate::tokens::{Token, TokenType};
use anyhow::Result;
use std::cell::RefCell;

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
