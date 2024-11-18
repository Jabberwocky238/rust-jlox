use std::{collections::HashMap, rc::Rc, vec};

use crate::{ast::LoxValue, errors::RuntimeError};

pub struct Environment {
    registry: Vec<HashMap<String, Rc<LoxValue>>>,
    ancestor: Vec<usize>,
    curregis: usize,
    curdepth: usize,
}

impl Environment {
    pub fn new() -> Environment {
        let body = Self {
            registry: vec![HashMap::new()],
            ancestor: vec![usize::MAX],
            curregis: 0,
            curdepth: 0,
        };
        return body;
    }

    pub fn enter_scope(&mut self, is_global: bool) {
        // dbg!("enter_scope");
        if is_global {
            self.ancestor.push(0);
        } else {
            self.ancestor.push(self.registry.len() - 1);
        }
        self.registry.push(HashMap::new());
        self.curdepth += 1;
        self.curregis = self.registry.len() - 1;
    }

    pub fn exit_scope(&mut self) {
        // dbg!("exit_scope");
        self.curdepth -= 1;
        self.curregis = self.ancestor[self.curregis];
    }

    pub fn define(&mut self, name: &str, value: Rc<LoxValue>) {
        self.registry[self.curregis].insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Result<Rc<LoxValue>, RuntimeError> {
        let mut regist_index = self.curregis;
        while regist_index != usize::MAX {
            // dbg!(&self.registry[regist_index]);
            // dbg!(&self.ancestor[regist_index]);
            // dbg!(&regist_index);
            if let Some(value) = self.registry[regist_index].get(name) {
                return Ok(value.clone());
            }
            regist_index = self.ancestor[regist_index];
        }
        Err(RuntimeError(
            "Undefined variable '".to_string() + name + "'.",
        ))
    }
    pub fn get_at(&self, distance: usize, name: &str) -> Result<Rc<LoxValue>, RuntimeError> {
        let mut regist_index = self.curregis;
        for _ in 0..distance {
            regist_index = self.ancestor[regist_index];
        }
        if let Some(value) = self.registry[regist_index].get(name) {
            return Ok(value.clone());
        }
        Err(RuntimeError(
            "Undefined variable '".to_string() + name + "'.",
        ))
    }

    pub fn assign(&mut self, name: &str, value: Rc<LoxValue>) -> Result<(), RuntimeError> {
        let mut regist_index = self.curregis;
        while regist_index != usize::MAX {
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
    pub fn assign_at(
        &mut self,
        distance: usize,
        name: &str,
        value: Rc<LoxValue>,
    ) -> Result<(), RuntimeError> {
        let mut regist_index = self.curregis;
        for _ in 0..distance {
            regist_index = self.ancestor[regist_index];
        }
        if self.registry[regist_index].contains_key(name) {
            self.registry[regist_index].insert(name.to_string(), value);
            return Ok(());
        }
        Err(RuntimeError(
            name.to_string() + "Undefined variable '" + name + "'.",
        ))
    }
}
