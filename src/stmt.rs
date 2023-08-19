use crate::environment::Environment;
use crate::errors::MyError;
use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::tokens::{Token, Type};
use anyhow::Result;
use std::iter::zip;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
    VarStmt {
        name: Token,
        initializer: Expr,
    },
    IfStmt {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Stmt>,
    },
    WhileStmt {
        condition: Expr,
        body: Box<Stmt>,
    },
    Block(Vec<Stmt>),
    Break,
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    NULL,
}

pub trait Callable {
    fn arity(&self) -> Result<usize>;
    fn call(&self, interpreter: &Interpreter, arguments: &[Type]) -> Result<()>;
}

// create a thin wrapper.
#[derive(Debug, Clone, PartialEq)]
pub struct LoxFunction {
    pub declaration: Stmt,
}

impl Callable for LoxFunction {
    fn arity(&self) -> Result<usize> {
        match &self.declaration {
            Stmt::Function { name, params, body } => Ok(params.len()),
            _ => Err(MyError::NotCallableError.into()),
        }
    }

    fn call(&self, interpreter: &Interpreter, arguments: &[Type]) -> Result<()> {
        let environment = Environment::new(Some(interpreter.globals.clone()));
        match &self.declaration {
            Stmt::Function { name, params, body } => {
                params
                    .iter()
                    .zip(arguments.iter())
                    .map(|(x, y)| environment.define(&x.lexeme, y))
                    .collect::<Vec<_>>();

                interpreter.execute_block(body, environment)
            }
            _ => Err(MyError::NotCallableError.into()),
        }
    }
}
