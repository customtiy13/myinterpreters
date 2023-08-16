use crate::expr::Expr;
use crate::tokens::Token;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
    VarStmt { name: Token, initializer: Expr },
}
