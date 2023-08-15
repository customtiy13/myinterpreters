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

//pub trait Expr: std::fmt::Debug {}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Literal(Option<String>),
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
}

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
    pub fn new(tokens: &[Token]) -> Self {
        Parser {
            tokens: tokens.into(),
            current: 0.into(),
        }
    }

    // TODO, catch error.
    pub fn parse(&self) -> Expr {
        self.expression()
    }

    fn expression(&self) -> Expr {
        self.equality()
    }

    fn equality(&self) -> Expr {
        self.binary_builder(
            &[TokenType::BangEqual, TokenType::EqualEqual],
            Self::comparsion,
        )
    }

    fn comparsion(&self) -> Expr {
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

    fn term(&self) -> Expr {
        self.binary_builder(&[TokenType::MINUS, TokenType::PLUS], Self::factor)
    }

    fn factor(&self) -> Expr {
        self.binary_builder(&[TokenType::SLASH, TokenType::STAR], Self::unary)
    }

    fn unary(&self) -> Expr {
        if self.is_match(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary {
                op: operator.clone(),
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&self) -> Expr {
        let operator = self.peek(0);
        self.advance();
        match operator.token_type {
            TokenType::FALSE => Expr::Literal(Some("false".into())),
            TokenType::TRUE => Expr::Literal(Some("true".into())),
            TokenType::NIL => Expr::Literal(Some("null".into())),
            TokenType::NUMBER | TokenType::STRING => Expr::Literal(self.previous().literal.clone()),
            TokenType::LeftParen => {
                let expr = self.expression();
                if let Err(e) = self.consume(TokenType::RightParen, "Expect ')' after expression.")
                {
                    self.error(self.peek(0), &e.to_string());
                    panic!();
                }
                Expr::Grouping(Box::new(expr))
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

    fn binary_builder<F>(&self, t: &[TokenType], op_method: F) -> Expr
    where
        F: Fn(&Self) -> Expr,
    {
        let mut expr = op_method(self);
        while self.is_match(t) {
            let operator = self.previous();
            let right = op_method(self);
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator.clone(),
                right: Box::new(right),
            }
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
