use crate::ast::{Function, StmtVisitable};
use crate::interpreter::Interpreter;
use crate::token::LoxValue;

pub trait LoxCallable: std::fmt::Display {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &Interpreter, arguments: Vec<LoxValue>) -> LoxValue;
}

// --------------------------------------------

pub struct LoxFunction<'a> {
    declaration: &'a Function,
}

impl<'a> LoxFunction<'a> {
    pub fn new(_declaration: &'a Function) -> Self {
        LoxFunction {
            declaration: _declaration,
        }
    }
}

impl<'a> LoxCallable for LoxFunction<'a> {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<LoxValue>) -> LoxValue {
        _interpreter.environment.borrow_mut().enter_scope(false);

        let mut _arguments = _arguments;
        let drain_arg = _arguments.drain(..).into_iter();
        for (param, arg) in self.declaration.params.iter().zip(drain_arg) {
            _interpreter.environment.borrow_mut().define(&param.lexeme, arg);
        }
        
        self.declaration.body.accept(_interpreter);

        _interpreter.environment.borrow_mut().exit_scope();
        LoxValue::Nil
    }
}

impl<'a> std::fmt::Display for LoxFunction<'a> {
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
    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<LoxValue>) -> LoxValue {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        LoxValue::Number(time)
    }
}

impl std::fmt::Display for BuiltinFunctioClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn clock>")
    }
}

// --------------------------------------------



