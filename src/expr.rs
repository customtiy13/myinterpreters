use crate::tokens::{Token, Type};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Literal(Type),
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Var(Token),
    Logical {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Null,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expr::*;
        fn expr_helper(expr: &Expr) -> String {
            match expr {
                Literal(expr) => expr.to_string(),
                Unary { op, right } => {
                    let op_str = op.lexeme.clone();
                    let right = expr_helper(right);
                    format!("{op_str}{right}")
                }
                Grouping(expr) => expr_helper(expr),
                Binary { left, op, right } => {
                    let left = expr_helper(left);
                    let op_str = op.lexeme.clone();
                    let right = expr_helper(right);
                    format!("{left}{op_str}{right}")
                }
                Var(token) => format!("{}", token.lexeme.clone()),
                Null => format!(""),
                Assign { name, value } => format!("{} = {}", name.lexeme, value),
                Logical { left, op, right } => {
                    format!("{} {} {}", left, op.lexeme.clone(), right)
                }
            }
        }

        let result: String = expr_helper(self);

        write!(f, "{}", result)
    }
}
