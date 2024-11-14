use std::any::{Any, TypeId};
use std::rc::Rc;

use crate::ast::{self, Function, LoxValue, StmtVisitable};
use crate::errors::RuntimeReturn;
use crate::interpreter::Interpreter;

pub trait LoxCallable: std::fmt::Display {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Rc<LoxValue>>) -> Rc<LoxValue>;
}

// --------------------------------------------

pub struct LoxFunction {
    declaration: Function,
}

impl LoxFunction {
    pub fn new(_declaration: Function) -> Self {
        LoxFunction {
            declaration: _declaration,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<Rc<LoxValue>>) -> Rc<LoxValue> {
        if _arguments.len() != self.arity() {
            panic!("Expected {} arguments but got {}.", self.arity(), _arguments.len());
        }
        
        _interpreter.environment.borrow_mut().enter_scope(false);
        
        let mut _arguments = _arguments;
        let drain_arg = _arguments.drain(..).into_iter();
        for (param, arg) in self.declaration.params.iter().zip(drain_arg) {
            _interpreter.environment.borrow_mut().define(&param.lexeme, arg);
        }
        
        let result = <ast::Stmt as Clone>::clone(&self.declaration.body).accept(_interpreter);

        _interpreter.environment.borrow_mut().exit_scope();

        match result {
            Err(err) => {
                if let Some(ret) = err.downcast_ref::<RuntimeReturn>() {
                    ret.0.clone()
                } else {
                    LoxValue::Nil.into()
                }
            }
            _ => LoxValue::Nil.into()
        }
    }
}

impl std::fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme)
    }
}

// --------------------------------------------

struct BuiltinFunctioClock;

pub fn builtin_function_clock() -> LoxValue {
    LoxValue::Callable(Box::new(BuiltinFunctioClock))
}

impl LoxCallable for BuiltinFunctioClock {
    fn arity(&self) -> usize {
        0
    }
    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<Rc<LoxValue>>) -> Rc<LoxValue> {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        LoxValue::Number(time).into()
    }
}

impl std::fmt::Display for BuiltinFunctioClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn clock>")
    }
}

// --------------------------------------------



