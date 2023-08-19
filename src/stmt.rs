use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::tokens::{Token, Type};

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
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &Interpreter, arguments: &[Type]) -> Type;
}

// create a thin wrapper.
#[derive(Debug, Clone, PartialEq)]
pub struct LoxFunction {
    pub declaration: Stmt,
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        todo!()
    }

    fn call(&self, interpreter: &Interpreter, arguments: &[Type]) -> Type {
        todo!()
    }
}
