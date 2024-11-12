use crate::token::Token;

#[derive(Debug)]
pub struct RuntimeError(pub String);

impl RuntimeError {
    pub fn new(token: &Token, message: &str) -> RuntimeError {
        RuntimeError(format!("RuntimeError at line {} column {}: {}", token.line, token.offset, message))
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

