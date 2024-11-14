use once_cell::sync::Lazy;
use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
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

#[macro_export]
macro_rules! op_to_tt {
    [+] => { TokenType::PLUS };
    [-] => { TokenType::MINUS };
    [*] => { TokenType::STAR };
    [/] => { TokenType::SLASH };
    [!] => { TokenType::BANG };
    [=] => { TokenType::EQUAL };
    [>] => { TokenType::GREATER };
    [<] => { TokenType::LESS };
    [>=] => { TokenType::GREATEREQUAL };
    [<=] => { TokenType::LESSEQUAL };
    [!=] => { TokenType::BANGEQUAL };
    [==] => { TokenType::EQUALEQUAL };
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
pub enum TokenLiteral {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl std::fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenLiteral::Number(n) => write!(f, "{}", n),
            TokenLiteral::String(s) => write!(f, "{}", s),
            TokenLiteral::Bool(b) => write!(f, "{}", b),
            TokenLiteral::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub _type: TokenType,
    pub lexeme: String,
    pub literal: TokenLiteral,
    pub line: usize,
    pub offset: usize,
}

impl Token {
    pub fn build(token_type: TokenType, lexeme: &str, literal: TokenLiteral, line: usize, offset: usize) -> Token {
        Token {
            _type: token_type,
            lexeme: String::from(lexeme),
            literal,
            line,
            offset,
        }
    }
}
