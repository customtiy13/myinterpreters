use crate::expr::Expr;
use crate::tokens::Token;

#[derive(Debug, PartialEq)]
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
    Block(Vec<Stmt>),
    NULL,
}
