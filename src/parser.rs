use crate::errors::MyError;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::tokens::{Token, TokenType, Type};
use anyhow::Result;
use log::debug;
use std::cell::RefCell;

const MAX_ARG_NUM: usize = 255;

/*
program        → declaration* EOF ;
declaration    → funDecl
               | varDecl
               | statement ;
funDecl        → "fun" function ;
function       → IDENTIFIER "(" parameters? ")" block ;
parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
statement      → exprStmt
               | printStmt ;
               | ifStmt
               | whileStmt
               | forStmt
whileStmt      → "while" "(" expression ")" statement ;
ifStmt         → "if" "(" expression ")" statement
forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
                 expression? ";"
                 expression? ")" statement ;
( "else" statement )? ;
               | block;
block          → "{" declaration* "}" ;
exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;
expression     → assignment ;
assignment     → IDENTIFIER "=" assignment
               | logic_or;
logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary | call ;
call           → primary ( "(" arguments? ")" )* ;
arguments      → expression ( "," expression )* ;
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
        if self.is_match(&[TokenType::FUN]) {
            return self.function("function");
        } else if self.is_match(&[TokenType::VAR]) {
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

    fn function(&self, kind: &str) -> Result<Stmt> {
        let name = self.consume(
            TokenType::IDENTIFIER,
            &format!("{}{}{}", "Expecet ", kind, " name."),
        )?;
        self.consume(
            TokenType::LeftParen,
            &format!("{}{}{}", "Expect '(' after ", kind, " name."),
        )?;
        let mut params = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if params.len() >= MAX_ARG_NUM {
                    return Err(MyError::MaxArgumentNumError.into());
                }

                params.push(
                    self.consume(TokenType::IDENTIFIER, "Expect parameter name.")?
                        .clone(),
                );
                if !self.is_match(&[TokenType::COMMA]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            TokenType::LeftBrace,
            &format!("{}{}{}", "Expect '{' before ", kind, " body."),
        )?;

        let body = match self.block_stmt()? {
            Stmt::Block(v) => v,
            _ => panic!("should not be here."),
        };

        Ok(Stmt::Function {
            name: name.clone(),
            params,
            body,
        })
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
        if self.is_match(&[TokenType::IF]) {
            return self.if_stmt();
        } else if self.is_match(&[TokenType::PRINT]) {
            return self.print_stmt();
        } else if self.is_match(&[TokenType::WHILE]) {
            return self.while_stmt();
        } else if self.is_match(&[TokenType::FOR]) {
            return self.for_stmt();
        } else if self.is_match(&[TokenType::LeftBrace]) {
            return self.block_stmt();
        } else if self.is_match(&[TokenType::BREAK]) {
            return self.break_stmt();
        }

        self.expr_stmt()
    }

    fn break_stmt(&self) -> Result<Stmt> {
        self.consume(TokenType::SEMICOLON, "Expect ';' after break.")?;

        Ok(Stmt::Break)
    }

    fn for_stmt(&self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
        // for (var a = 2; a < 3; a = a + 1)

        // initializer
        let initializer = if self.is_match(&[TokenType::SEMICOLON]) {
            Stmt::NULL
        } else if self.is_match(&[TokenType::VAR]) {
            debug!("In for stmt: var initializer");
            self.var_declaration()?
        } else {
            self.statement()?
        };

        // condition.
        let mut condition = if self.check(&TokenType::SEMICOLON) {
            Expr::Null
        } else {
            self.expression()?
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition.")?;

        // increment
        let increment = if self.check(&TokenType::RightParen) {
            Expr::Null
        } else {
            self.expression()?
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        // body
        let mut body = self.statement()?;

        // iteriting backward. assuming each one is null.
        body = match increment {
            Expr::Null => body, // no increnment.
            _ => Stmt::Block(vec![body, Stmt::ExprStmt(increment)]),
        };

        // condition case.
        if let Expr::Null = condition {
            condition = Expr::Literal(Type::Bool(true));
        }
        body = Stmt::WhileStmt {
            condition,
            body: Box::new(body),
        };

        // initializer case.
        body = match initializer {
            Stmt::NULL => body,
            _ => Stmt::Block(vec![initializer, body]),
        };

        Ok(body)
    }

    fn while_stmt(&self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after while.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;
        let body = self.statement()?;

        Ok(Stmt::WhileStmt {
            condition,
            body: Box::new(body),
        })
    }

    fn if_stmt(&self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let else_branch = if self.is_match(&[TokenType::ELSE]) {
            self.statement()?
        } else {
            Stmt::NULL
        };

        Ok(Stmt::IfStmt {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        })
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
        let expr = self.or()?;
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

    fn or(&self) -> Result<Expr> {
        let mut expr = self.and()?;

        while self.is_match(&[TokenType::OR]) {
            let op = self.previous();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op: op.clone(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.is_match(&[TokenType::AND]) {
            let op = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op: op.clone(),
                right: Box::new(right),
            };
        }

        Ok(expr)
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

        self.call()
    }

    fn call(&self) -> Result<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.is_match(&[TokenType::LeftParen]) {
                expr = self.finish_call(&expr)?
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&self, expr: &Expr) -> Result<Expr> {
        let mut arguments = Vec::new();
        if !self.check(&TokenType::RightParen) {
            //have arguments.
            loop {
                if arguments.len() >= MAX_ARG_NUM {
                    return Err(MyError::MaxArgumentNumError.into());
                }
                arguments.push(self.expression()?);
                if !self.is_match(&[TokenType::COMMA]) {
                    // last one.
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call {
            callee: Box::new(expr.clone()),
            paren: paren.clone(),
            arguments,
        })
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
