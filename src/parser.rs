use crate::errors::MyError;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::tokens::{Token, TokenType, Type};
use anyhow::Result;
use std::cell::RefCell;

/*
program        → declaration* EOF ;
declaration    -> varDecl
               | statement;
varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
statement      → exprStmt
               | printStmt ;
               | block;
block          → "{" declaration* "}" ;
exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;
expression     → assignment ;
assignment     → IDENTIFIER "=" assignment
               | equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
               | IDENTIFIER
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

    pub fn parse(&self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.is_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&self) -> Result<Stmt> {
        if self.is_match(&[TokenType::VAR]) {
            match self.var_declaration() {
                Err(e) => {
                    //self.synchronize();
                    return Err(e);
                }
                Ok(v) => {
                    return Ok(v);
                }
            }
        } else {
            self.statement()
        }
    }

    fn var_declaration(&self) -> Result<Stmt> {
        let name = self.consume(TokenType::IDENTIFIER, "Expected variable name.")?;
        let mut initializer = Expr::Null;
        if self.is_match(&[TokenType::EQUAL]) {
            initializer = self.expression()?;
        }

        self.consume(
            TokenType::SEMICOLON,
            "Expected ';' after variable declaration.",
        )?;

        Ok(Stmt::VarStmt {
            name: name.clone(),
            initializer,
        })
    }

    fn statement(&self) -> Result<Stmt> {
        if self.is_match(&[TokenType::PRINT]) {
            return self.print_stmt();
        } else if self.is_match(&[TokenType::LeftBrace]) {
            return self.block_stmt();
        }

        self.expr_stmt()
    }

    fn block_stmt(&self) -> Result<Stmt> {
        let mut statements = Vec::new();

        while !self.is_end() && !self.check(&TokenType::RightBrace) {
            statements.push(self.declaration()?)
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block.")?;

        Ok(Stmt::Block(statements))
    }

    fn print_stmt(&self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value")?;

        Ok(Stmt::PrintStmt(value))
    }

    fn expr_stmt(&self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value")?;

        Ok(Stmt::ExprStmt(expr))
    }

    fn expression(&self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&self) -> Result<Expr> {
        let expr = self.equality()?;
        if self.is_match(&[TokenType::EQUAL]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Var(t) = expr {
                let name: Token = t;
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            } else {
                return Err(MyError::InvalidAssignmentTargetError(equals.lexeme.clone()).into());
            }
        }

        return Ok(expr);
    }

    fn equality(&self) -> Result<Expr> {
        self.binary_builder(
            &[TokenType::BangEqual, TokenType::EqualEqual],
            Self::comparsion,
        )
    }

    fn comparsion(&self) -> Result<Expr> {
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

    fn term(&self) -> Result<Expr> {
        self.binary_builder(&[TokenType::MINUS, TokenType::PLUS], Self::factor)
    }

    fn factor(&self) -> Result<Expr> {
        self.binary_builder(&[TokenType::SLASH, TokenType::STAR], Self::unary)
    }

    fn unary(&self) -> Result<Expr> {
        if self.is_match(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                op: operator.clone(),
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&self) -> Result<Expr> {
        let operator = self.peek(0);
        self.advance();
        match operator.token_type {
            TokenType::FALSE => Ok(Expr::Literal(Type::Bool(false))),
            TokenType::TRUE => Ok(Expr::Literal(Type::Bool(true))),
            TokenType::NIL => Ok(Expr::Literal(Type::Nil)),
            TokenType::NUMBER | TokenType::STRING => {
                Ok(Expr::Literal(self.previous().literal.clone()))
            }
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            TokenType::IDENTIFIER => Ok(Expr::Var(self.previous().clone())),
            _ => {
                self.error(self.peek(0), "Expected expression.");
                Err(MyError::NotImplementedError.into())
            }
        }
    }

    fn consume(&self, t: TokenType, msg: &str) -> Result<&Token> {
        if self.check(&t) {
            let current = self.peek(0);
            self.advance();
            return Ok(current);
        }

        Err(MyError::ParseError(msg.into()).into())
    }

    fn binary_builder<F>(&self, t: &[TokenType], op_method: F) -> Result<Expr>
    where
        F: Fn(&Self) -> Result<Expr>,
    {
        let mut expr = op_method(self)?;
        while self.is_match(t) {
            let operator = self.previous();
            let right = op_method(self)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator.clone(),
                right: Box::new(right),
            }
        }

        Ok(expr)
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
        self.peek(0).token_type == TokenType::EOF || *self.current.borrow() >= self.tokens.len() - 1
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
