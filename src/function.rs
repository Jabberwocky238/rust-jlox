use crate::ast::Function;
use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::token::{LoxLiteralValue, LoxValue};

pub trait LoxCallable: std::fmt::Display {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> LoxValue;
}

pub struct LoxFunction {
    declaration: Function,
}

impl LoxFunction {
    pub fn new(_de: Function) -> Self {
        LoxFunction {
            declaration: _de,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        0
    }
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<LoxValue>) -> LoxValue {
        let environment = Environment::new();
        _interpreter.environment.borrow().push(environment);
        for (param, arg) in self.declaration.params.iter().zip(_arguments.iter()) {
            _interpreter.environment.borrow().peek().define(param.lexeme.clone(), arg.clone());
        }
        let _ = _interpreter.execute_block(&self.declaration.body, _interpreter.environment.borrow().peek());
        _interpreter.environment.borrow().pop();
        LoxValue::Literal(LoxLiteralValue::Nil)
    }
}

impl std::fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme)
    }
}

pub struct BuiltinFunctioClock;

impl LoxCallable for BuiltinFunctioClock {
    fn arity(&self) -> usize {
        0
    }
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<LoxValue>) -> LoxValue {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        LoxValue::Literal(LoxLiteralValue::Number(time))
    }
}

impl std::fmt::Display for BuiltinFunctioClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn clock>")
    }
}