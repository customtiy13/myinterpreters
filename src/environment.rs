use crate::errors::MyError;
use crate::tokens::Token;
use crate::tokens::Type;
use anyhow::Result;
use std::collections::HashMap;

//TODO
#[derive(Debug)]
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
            //TODO runtime error
            None => Err(MyError::EnValueNotFoundError(name.lexeme.clone()).into()),
        }
    }
}
