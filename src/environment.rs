use std::{cell::{Cell, RefCell}, collections::HashMap, ops::DerefMut, rc::Rc};
use std::ops::Deref;
use crate::{errors::RuntimeError, scanner::LiteralType};

// #[derive(Clone)]
pub struct Environment {
    pub enclosing: RefCell<Option<Rc<Environment>>>,
    pub values: HashMap<String, LiteralType>,
}



impl Deref for Environment {
    type Target = Environment;

    fn deref(&self) -> &Self::Target {
        self
    }
}

impl DerefMut for Environment {
    fn deref_mut(&mut self) -> &mut Self {
        self
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: RefCell::new(None),
        }
    }
    pub fn build(_enclosing: Rc<Environment>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: RefCell::new(Some(_enclosing)),
        }
    }

    pub fn define(&mut self, name: &str, value: LiteralType) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Result<LiteralType, RuntimeError> {
        match self.values.get(name) {
            Some(value) => Ok(value.clone()),
            None => {
                match self.enclosing.borrow().as_ref() {
                    Some(enclosing) => {
                        return Ok(enclosing.get(name).unwrap().clone());
                    },
                    None => Err(RuntimeError(
                        name.to_string() + "Undefined variable '" + name + "'.",
                    ))
                }
            },
        }
    }

    pub fn assign(&mut self, name: &str, value: LiteralType) -> Result<(), RuntimeError> {
        match self.values.get_mut(name) {
            Some(v) => {
                *v = value;
                Ok(())
            }
            None => {
                match self.enclosing.borrow().as_ref() {
                    Some(enclosing) => enclosing.deref_mut().assign(name, value),
                    None => Err(RuntimeError(
                        name.to_string() + "Undefined variable '" + name + "'.",
                    ))
                }
            }
        }
    }
}
