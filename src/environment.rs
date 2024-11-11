use std::{collections::HashMap, vec};

use crate::{errors::RuntimeError, scanner::LiteralType};

#[derive(Debug)]
pub struct Environment {
    pub stack: Vec<HashMap<String, LiteralType>>,
    pub depth: usize,
}

impl Environment {
    pub fn new() -> Environment {
        let body = Self {
            stack: vec![HashMap::new()],
            depth: 0,
        };
        return body;
    }

    pub fn enter_scope(&mut self) {
        self.stack.push(HashMap::new());
        self.depth += 1;
    }

    pub fn exit_scope(&mut self) {
        self.stack.pop();
        self.depth -= 1;
    }

    pub fn define(&mut self, name: &str, value: LiteralType) {
        self.stack[self.depth].insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Result<LiteralType, RuntimeError> {
        for i in (0..=self.depth).rev() {
            if let Some(value) = self.stack[i].get(name) {
                return Ok(value.clone());
            }
        }
        Err(RuntimeError(
            name.to_string() + "Undefined variable '" + name + "'.",
        ))
    }

    pub fn assign(&mut self, name: &str, value: LiteralType) -> Result<(), RuntimeError> {
        for i in (0..=self.depth).rev() {
            if self.stack[i].contains_key(name) {
                self.stack[i].insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(RuntimeError(
            name.to_string() + "Undefined variable '" + name + "'.",
        ))
    }
}
