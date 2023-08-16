use crate::errors::MyError;
use crate::expr::Expr;
use crate::tokens::{TokenType, Type};
use anyhow::Result;
pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }
    pub fn evaluate(&self, expr: &Expr) -> Result<Type> {
        use Expr::*;
        match expr {
            Literal(value) => Ok(value.clone()),
            Binary { left, op, right } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;
                match op.token_type {
                    // a bit ugly. refactor this.
                    TokenType::MINUS
                    | TokenType::SLASH
                    | TokenType::STAR
                    | TokenType::GREATER
                    | TokenType::GreaterEqual
                    | TokenType::LESS
                    | TokenType::LessEqual => {
                        let lnum = self.get_number(&left, op.line)?;
                        let rnum = self.get_number(&right, op.line)?;

                        match op.token_type {
                            TokenType::MINUS => Ok(Type::Number(lnum - rnum)),
                            TokenType::SLASH => {
                                if rnum == 0.0 {
                                    return Err(MyError::DividedbyzeroError.into());
                                }
                                Ok(Type::Number(lnum / rnum))
                            }
                            TokenType::STAR => Ok(Type::Number(lnum * rnum)),
                            TokenType::GREATER => Ok(Type::Bool(lnum > rnum)),
                            TokenType::GreaterEqual => Ok(Type::Bool(lnum >= rnum)),
                            TokenType::LESS => Ok(Type::Bool(lnum < rnum)),
                            TokenType::LessEqual => Ok(Type::Bool(lnum <= rnum)),
                            _ => panic!("never reach this."),
                        }
                    }
                    TokenType::BangEqual => Ok(Type::Bool(!self.is_equal(&left, &right))),
                    TokenType::EqualEqual => Ok(Type::Bool(self.is_equal(&left, &right))),
                    TokenType::PLUS => {
                        // 1. add num 2. concat strings.
                        match (left, right) {
                            (Type::Number(lnum), Type::Number(rnum)) => {
                                Ok(Type::Number(lnum + rnum))
                            }
                            (Type::String(lstr), Type::String(rstr)) => {
                                Ok(Type::String(format!("{}{}", lstr, rstr)))
                            }
                            _ => {
                                let line = op.line;
                                Err(MyError::CastError(format!(
                                    "[Line {line}]: Operation not supported."
                                ))
                                .into())
                            }
                        }
                    }
                    _ => {
                        panic!("TODO Binary op")
                    }
                }
            }
            Grouping(expr) => self.evaluate(expr),
            Unary { op, right } => {
                let right = self.evaluate(right)?;
                match op.token_type {
                    TokenType::BANG => Ok(Type::Bool(!self.is_truthy(&right))),
                    TokenType::MINUS => {
                        // could only be number
                        let num = self.get_number(&right, op.line)?;
                        Ok(Type::Number(-num))
                    }
                    _ => todo!(),
                }
            }
            _ => {
                todo!()
            }
        }
    }

    fn get_number(&self, t: &Type, line: usize) -> Result<f64> {
        match t {
            Type::Number(value) => Ok(*value),
            _ => {
                let msg = format!("[Line {line}]: Operand must be a number");
                Err(MyError::CastError(msg).into())
            }
        }
    }

    fn is_truthy(&self, t: &Type) -> bool {
        match *t {
            Type::Nil => false,
            Type::Bool(value) => value,
            _ => true,
        }
    }

    fn is_equal(&self, left: &Type, right: &Type) -> bool {
        match (left, right) {
            (Type::Nil, Type::Nil) => true,
            (Type::Nil, _) | (_, Type::Nil) => false,
            _ => left == right,
        }
    }
}
