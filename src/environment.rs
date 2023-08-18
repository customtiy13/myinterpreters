use crate::errors::MyError;
use crate::tokens::Token;
use crate::tokens::Type;
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

//TODO
#[derive(Debug, PartialEq)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: RefCell<HashMap<String, Type>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            enclosing,
            values: HashMap::new().into(),
        }
    }

    pub fn define(&self, name: &str, value: &Type) {
        self.values
            .borrow_mut()
            .insert(name.to_string(), value.clone());
    }

    pub fn set_env(&mut self, environment: Rc<RefCell<Environment>>) {
        self.enclosing = Some(environment.clone());
    }

    pub fn get(&self, name: &Token) -> Result<Type> {
        match self.values.borrow().get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => {
                // enclosing get.
                if let Some(ref v) = self.enclosing {
                    return v.borrow().get(name);
                }

                Err(MyError::EnValueNotFoundError(name.lexeme.clone()).into())
            }
        }
    }

    pub fn assign(&self, name: &Token, value: &Type) -> Result<()> {
        match self.values.borrow_mut().get_mut(&name.lexeme) {
            Some(v) => {
                *v = value.clone();

                Ok(())
            }
            None => {
                if let Some(ref v) = self.enclosing {
                    return v.borrow().assign(name, value);
                }

                Err(MyError::EnValueNotFoundError(name.lexeme.clone()).into())
            }
        }
    }
}
