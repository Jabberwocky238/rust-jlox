use once_cell::sync::Lazy;
use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::string::String;
use std::sync::Mutex;

use crate::function::LoxCallable;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LEFTPAREN,  // (
    RIGHTPAREN, // )
    LEFTBRACE,  // {
    RIGHTBRACE, // }
    COMMA,      // ,
    DOT,        // .
    MINUS,      // -
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG, // !
    BANGEQUAL, // !=
    EQUAL, // =
    EQUALEQUAL, // ==
    GREATER, // >
    GREATEREQUAL, // >=
    LESS, // <
    LESSEQUAL, // <=
    AND, // and
    OR, // or

    // Literals.
    IDENTIFIER, 
    STRING,
    NUMBER,

    // Keywords.
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

pub static KEYWORDS: Lazy<Mutex<HashMap<String, TokenType>>> = Lazy::new(|| {
    let m = HashMap::from([
        ("and".to_string(), TokenType::AND),
        ("class".to_string(), TokenType::CLASS),
        ("else".to_string(), TokenType::ELSE),
        ("false".to_string(), TokenType::FALSE),
        ("for".to_string(), TokenType::FOR),
        ("fun".to_string(), TokenType::FUN),
        ("if".to_string(), TokenType::IF),
        ("nil".to_string(), TokenType::NIL),
        ("or".to_string(), TokenType::OR),
        ("print".to_string(), TokenType::PRINT),
        ("return".to_string(), TokenType::RETURN),
        ("super".to_string(), TokenType::SUPER),
        ("this".to_string(), TokenType::THIS),
        ("true".to_string(), TokenType::TRUE),
        ("var".to_string(), TokenType::VAR),
        ("while".to_string(), TokenType::WHILE),
    ]);
    Mutex::new(m)
});

#[derive(Debug, Clone, PartialEq)]
pub enum LoxLiteral {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl std::fmt::Display for LoxLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxLiteral::Number(n) => write!(f, "{}", n),
            LoxLiteral::String(s) => write!(f, "{}", s),
            LoxLiteral::Bool(b) => write!(f, "{}", b),
            LoxLiteral::Nil => write!(f, "nil"),
        }
    }
}

pub enum LoxValue {
    Literal(LoxLiteral),
    Callable(Box<dyn LoxCallable>),
}

impl std::fmt::Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxValue::Literal(l) => write!(f, "{}", l),
            LoxValue::Callable(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub _type: TokenType,
    pub lexeme: String,
    pub literal: LoxLiteral,
    pub line: usize,
    pub offset: usize,
}

impl Token {
    pub fn build(token_type: TokenType, lexeme: &str, literal: LoxLiteral, line: usize, offset: usize) -> Token {
        Token {
            _type: token_type,
            lexeme: String::from(lexeme),
            literal,
            line,
            offset,
        }
    }
}
