use std::collections::HashMap;

use crate::tokens::TokenType;
#[derive(Default, Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, TokenType>,
}
impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: TokenType) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: &str) -> Option<&TokenType> {
        self.values.get(name).or_else(|| {
            self.enclosing
                .as_ref()
                .and_then(|enclosing| enclosing.get(name))
        })
    }
    pub fn assign(&mut self, name: &str, value: TokenType) -> Option<()> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Some(())
        } else {
            self.enclosing
                .as_mut()
                .and_then(|enclosing| enclosing.assign(name, value))
                .or(None)
        }
    }
}
