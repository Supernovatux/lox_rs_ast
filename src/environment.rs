use std::collections::{HashMap, VecDeque};

use crate::tokens::TokenType;
#[derive(Default, Clone, Debug)]
pub struct Environment {
    values: VecDeque<HashMap<String, TokenType>>,
}
impl Environment {
    pub fn new() -> Self {
        let mut values = VecDeque::new();
        values.push_front(HashMap::new());
        Self { values }
    }
    pub fn define(&mut self, name: String, value: TokenType) {
        self.values.front_mut().unwrap().insert(name, value);
    }
    pub fn get(&self, name: &str) -> Option<TokenType> {
        for i in &self.values {
            if i.contains_key(name) {
                let val = i.get(name).unwrap().clone();
                return Some(val);
            }
        }
        None
    }
    pub fn assign(&mut self, name: &str, value: TokenType) -> Option<()> {
        for i in self.values.iter_mut() {
            if i.contains_key(name) {
                i.insert(name.to_string(), value);
                return Some(());
            }
        }
        None
    }
    pub fn enter_scope(&mut self) {
        self.values.push_front(HashMap::new());
    }
    pub fn exit_scope(&mut self) {
        self.values.pop_front();
    }
}
