use crate::environment::Environment;
use crate::errors::MyError;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::tokens::{TokenType, Type};
use anyhow::Result;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    is_REPL: RefCell<bool>,
}

impl Interpreter {
    pub fn new(is_REPL: bool) -> Self {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new(None))),
            is_REPL: RefCell::new(is_REPL),
        }
    }

    pub fn interpret(&self, stmts: &[Stmt]) -> Result<()> {
        for stmt in stmts {
            self.evaluate_stmt(stmt)?;
        }

        Ok(())
    }

    fn evaluate_stmt(&self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::ExprStmt(expr) => {
                let result = self.evaluate_expr(expr)?;
                if self.is_REPL() {
                    println!("{}", result);
                }
            }
            Stmt::PrintStmt(expr) => {
                let value = self.evaluate_expr(expr)?;
                println!("{}", value);
            }
            var @ Stmt::VarStmt {
                name: _,
                initializer: _,
            } => {
                self.define_var_stmt(var)?;
            }
            Stmt::Block(vec) => {
                self.execute_block(vec)?;
            }
            Stmt::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => {
                let expr_result = self.evaluate_expr(condition)?;
                if self.is_truthy(&expr_result) {
                    self.evaluate_stmt(then_branch)?;
                } else {
                    self.evaluate_stmt(else_branch)?;
                }
            }
            Stmt::WhileStmt { condition, body } => {
                while self.is_truthy(&self.evaluate_expr(condition)?) {
                    self.evaluate_stmt(body)?;
                }
            }
            Stmt::NULL => {
                // skip. nothing to be done.
            }
        };

        Ok(())
    }

    fn execute_block(&self, statements: &[Stmt]) -> Result<()> {
        // [Attention] Be careful when swapping env. Bad things can happen.
        let new_environment = Environment::new(None);
        // swap in the new environment.
        let pre_env = Rc::new(RefCell::new(self.environment.replace(new_environment)));
        self.set_env(pre_env.clone());
        self.interpret(statements)?;
        // swap back.
        self.environment.swap(&pre_env);

        Ok(())
    }

    fn define_var_stmt(&self, stmt: &Stmt) -> Result<()> {
        let (name, value) = match stmt {
            Stmt::VarStmt { name, initializer } => (name, self.evaluate_expr(initializer)?),
            _ => panic!("should not be here."),
        };

        self.environment.borrow_mut().define(&name.lexeme, &value);
        //println!("{:?}", self.environment);

        Ok(())
    }

    fn get_var_expr(&self, expr: &Expr) -> Result<Type> {
        //println!("{:?}", self.environment);
        let value = match expr {
            Expr::Var(ref token) => self.environment.borrow().get(token)?,
            _ => panic!("should not be here."),
        };

        Ok(value.clone())
    }

    pub fn evaluate_expr(&self, expr: &Expr) -> Result<Type> {
        use Expr::*;
        match expr {
            Literal(value) => Ok(value.clone()),
            Binary { left, op, right } => {
                let left = self.evaluate_expr(left)?;
                let right = self.evaluate_expr(right)?;
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
            Grouping(expr) => self.evaluate_expr(expr),
            Unary { op, right } => {
                let right = self.evaluate_expr(right)?;
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
            Null => Ok(Type::Nil),
            var @ Var(_) => Ok(self.get_var_expr(var)?),
            Assign { name, value } => {
                let value = self.evaluate_expr(value)?;
                self.environment.borrow_mut().assign(&name, &value)?;
                Ok(value)
            }
            Logical { left, op, right } => {
                // short circut.
                let left = self.evaluate_expr(left)?;

                match op.token_type {
                    TokenType::OR => {
                        if self.is_truthy(&left) {
                            return Ok(left);
                        } else {
                            return self.evaluate_expr(right);
                        }
                    }
                    TokenType::AND => {
                        if !self.is_truthy(&left) {
                            return Ok(left);
                        } else {
                            return self.evaluate_expr(right);
                        }
                    }
                    _ => {
                        panic!("not logical Operand");
                    }
                }
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

    fn set_env(&self, environment: Rc<RefCell<Environment>>) {
        self.environment.borrow_mut().set_env(environment);
    }

    pub fn get_environment(&self) -> Result<&RefCell<Environment>> {
        Ok(&self.environment)
    }

    fn is_REPL(&self) -> bool {
        *self.is_REPL.borrow()
    }
}
