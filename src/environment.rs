use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc, sync::RwLock};
use once_cell::sync::Lazy;

use crate::{errors::RuntimeError, scanner::LiteralType};

#[derive(Debug)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, LiteralType>,
    pub uid: u64,
}

static mut ENV_ID: Lazy<RwLock<u64>> = Lazy::new(|| {
    RwLock::new(0)
});

fn rand() -> u64 {
    let mut id = unsafe { ENV_ID.write().unwrap() };
    *id += 1;
    return *id;
}

impl Environment {
    pub fn new(_enclosing: Option<Rc<RefCell<Environment>>>) -> RefCell<Environment> {
        let body = Self {
            values: HashMap::new(),
            enclosing: _enclosing,
            uid: rand(),
        };
        let body = RefCell::new(body);

        if let Some(x) = &body.borrow().enclosing {
            assert_ne!(body.borrow().uid, x.as_ref().borrow().uid);
        }
        return body;
    }

    pub fn define(&mut self, name: &str, value: LiteralType) {
        
        if let Some(x) = &self.enclosing {
            assert_ne!(self.uid, x.as_ref().borrow().uid);
            
        }
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Result<LiteralType, RuntimeError> {
        if let Some(x) = &self.enclosing {
            assert_ne!(self.uid, x.as_ref().borrow().uid);
        }
        if self.values.contains_key(name) {
            return Ok(self.values.get(name).unwrap().clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.as_ref().borrow().get(name);
        }
        Err(RuntimeError(
            name.to_string() + "Undefined variable '" + name + "'.",
        ))
    }

    pub fn assign(&mut self, name: &str, value: LiteralType) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            return Ok(());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.as_ref().borrow_mut().assign(name, value);
        }
        Err(RuntimeError(
            name.to_string() + "Undefined variable '" + name + "'.",
        ))
    }
}

// #[test]
// fn t_env() -> Result<(), RuntimeError> {
//     let env = Rc::new(Environment::new());
//     env.borrow_mut().define("a", LiteralType::Number(1.0));
//     assert_eq!(env.as_ref().borrow().get("a").unwrap(), LiteralType::Number(1.0));
//     env.borrow_mut().assign("a", LiteralType::Number(2.0))?;
//     assert_eq!(env.as_ref().borrow().get("a").unwrap(), LiteralType::Number(2.0));

//     let env2 = Environment::build(env.clone());
//     env2.borrow_mut().define("b", LiteralType::Number(3.0));
//     assert_eq!(env2.borrow().get("b").unwrap(), LiteralType::Number(3.0));
//     env2.borrow_mut().assign("b", LiteralType::Number(4.0))?;
//     assert_eq!(env2.borrow().get("b").unwrap(), LiteralType::Number(4.0));
//     assert_eq!(env2.borrow().get("a").unwrap(), LiteralType::Number(2.0));
//     env2.borrow_mut().assign("a", LiteralType::Number(5.0))?;
//     assert_eq!(env2.borrow().get("a").unwrap(), LiteralType::Number(5.0));

//     let env3 = Environment::build(env.clone());
//     env3.borrow_mut().assign("a", LiteralType::Number(9.0))?;
//     assert_eq!(env3.borrow().get("a").unwrap(), LiteralType::Number(9.0));
//     assert_eq!(env2.borrow().get("a").unwrap(), LiteralType::Number(9.0));
//     assert_eq!(env.as_ref().borrow().get("a").unwrap(), LiteralType::Number(9.0));

//     Ok(())
// }