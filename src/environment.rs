use crate::errors::MyError;
use crate::tokens::Token;
use crate::tokens::Type;
use anyhow::Result;
use std::collections::HashMap;

//TODO
#[derive(Debug, PartialEq)]
pub struct Environment {
    values: HashMap<String, Type>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: &Type) {
        self.values.insert(name.to_string(), value.clone());
    }

    pub fn get(&self, name: &Token) -> Result<&Type> {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v),
            None => Err(MyError::EnValueNotFoundError(name.lexeme.clone()).into()),
        }
    }

    pub fn assign(&mut self, name: &Token, value: &Type) -> Result<()> {
        match self.values.get_mut(&name.lexeme) {
            Some(v) => *v = value.clone(),
            None => {
                return Err(MyError::EnValueNotFoundError(name.lexeme.clone()).into());
            }
        }

        Ok(())
    }
}
