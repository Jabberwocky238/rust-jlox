use crate::token::{LoxValue, Token};
use std::fmt::{Debug, Display, Formatter, Result};

pub trait RuntimeErrorT: Display{}

// -------------------------------------------------------
pub struct RuntimeError(pub String);

impl RuntimeErrorT for RuntimeError{}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl RuntimeError {
    pub fn new(token: &Token, message: &str) -> RuntimeError {
        RuntimeError(format!("RuntimeError at line {} column {}: {}", token.line, token.offset, message))
    }
}

// -------------------------------------------------------
pub struct RuntimeReturn(pub LoxValue);

impl RuntimeErrorT for RuntimeReturn{}

impl Display for RuntimeReturn {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "LoxFunctionReturn")
    }
}

impl Debug for RuntimeReturn {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl RuntimeReturn {
    pub fn new(_value: LoxValue) -> RuntimeReturn {
        RuntimeReturn(_value)
    }
}




#[derive(Debug)]
pub struct ParseError(pub String);

impl ParseError {
    pub fn new(token: Option<&Token>, message: &str) -> ParseError {
        let token = token.unwrap();
        ParseError(format!("ParseError at line {} column {}: {}", token.line, token.offset, message))
    }
}

