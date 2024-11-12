use std::{collections::HashMap, vec};

use crate::{errors::RuntimeError, token::LoxValue};

pub struct Environment<'env> {
    registry: Vec<HashMap<String, LoxValue<'env>>>,
    ancestor: Vec<usize>,
    curregis: usize,
    curdepth: usize,
}

impl<'env> Environment<'env> {
    pub fn new() -> Environment<'env> {
        let body = Self {
            registry: vec![HashMap::new()],
            ancestor: vec![usize::MAX],
            curregis: 0,
            curdepth: 0,
        };
        return body;
    }
    
    pub fn enter_scope(&mut self, is_global: bool) {
        self.registry.push(HashMap::new());
        if is_global {
            self.ancestor.push(0);
        } else {
            self.ancestor.push(self.ancestor[self.registry.len() - 1]);
        }
        self.curdepth += 1;
        self.curregis = self.registry.len() - 1;
    }

    pub fn exit_scope(&mut self) {
        self.curdepth -= 1;
        self.curregis = self.ancestor[self.ancestor.len() - 1];
    }

    pub fn define(&mut self, name: &str, value: LoxValue<'env>) {
        self.registry[self.curregis].insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Result<&LoxValue, RuntimeError> {
        let mut regist_index = self.curregis;
        for depth in 0..self.curdepth {
            if let Some(value) = self.registry[regist_index].get(name) {
                return Ok(value);
            }
            regist_index = self.ancestor[regist_index];
        }
        Err(RuntimeError(
            name.to_string() + "Undefined variable '" + name + "'.",
        ))
    }

    pub fn assign(&mut self, name: &str, value: LoxValue<'env>) -> Result<(), RuntimeError> {
        let mut regist_index = self.curregis;
        for depth in 0..self.curdepth {
            if self.registry[regist_index].contains_key(name) {
                self.registry[regist_index].insert(name.to_string(), value);
                return Ok(());
            }
            regist_index = self.ancestor[regist_index];
        }
        Err(RuntimeError(
            name.to_string() + "Undefined variable '" + name + "'.",
        ))
    }
}
