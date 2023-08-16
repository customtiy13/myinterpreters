use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
}
