use std::collections::HashMap;

use crate::tokens::TokenType;
#[derive(Default, Clone, Debug)]
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
        self.values.insert(name.clone(), value.clone());
        println!("define {} {}", name, value);
    }
    pub fn get(&self, name: &str) -> Option<&TokenType> {
        let val = self.values.get(name).or_else(|| {
            self.enclosing
                .as_ref()
                .and_then(|enclosing| enclosing.get(name))
        });
        val
    }
    pub fn assign(&mut self, name: &str, value: TokenType) -> Option<()> {
        println!("Self values {:?}", self.values);
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value.clone());
            println!("{} {:?}", value, self.get(name));
            Some(())
        } else {
            println!("{:?}", self.enclosing);
            self.enclosing
                .as_mut()
                .and_then(|enclosing| enclosing.assign(name, value))
                .or(None)
        }
    }
}
