use once_cell::sync::Lazy;
use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::string::String;
use std::sync::Mutex;

use super::logger::Logger;

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

    // Literals.
    IDENTIFIER, 
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

static KEYWORDS: Lazy<Mutex<HashMap<String, TokenType>>> = Lazy::new(|| {
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

#[derive(Debug, Clone)]
pub enum LiteralType {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl From<LiteralType> for String {
    fn from(value: LiteralType) -> Self {
        if let LiteralType::String(s) = value {
            s
        } else if let LiteralType::Number(n) = value {
            n.to_string()
        } else if let LiteralType::Boolean(b) = value {
            b.to_string()
        } else {
            "nil".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub _type: TokenType,
    pub lexeme: String,
    pub literal: LiteralType,
    pub line: usize,
    pub offset: usize,
}

impl Token {
    fn build(token_type: TokenType, lexeme: &str, literal: LiteralType, line: usize, offset: usize) -> Token {
        Token {
            _type: token_type,
            lexeme: String::from(lexeme),
            literal,
            line,
            offset,
        }
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn build(_source: &String) -> Scanner {
        Scanner {
            source: _source.clone(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        // self.source.split(" ").collect::<Vec<&str>>()
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::build(
            TokenType::EOF,
            "",
            LiteralType::String("".to_owned()),
            self.line,
            self.current,
        ));
        return &self.tokens;
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn scan_token(&mut self) -> Option<bool> {
        if let Some(c) = self._advance() {
            match c {
                '(' => self._add_token(TokenType::LEFTPAREN),
                ')' => self._add_token(TokenType::RIGHTPAREN),
                '{' => self._add_token(TokenType::LEFTBRACE),
                '}' => self._add_token(TokenType::RIGHTBRACE),
                ',' => self._add_token(TokenType::COMMA),
                '.' => self._add_token(TokenType::DOT),
                '-' => self._add_token(TokenType::MINUS),
                '+' => self._add_token(TokenType::PLUS),
                ';' => self._add_token(TokenType::SEMICOLON),
                '*' => self._add_token(TokenType::STAR),
                '"' => self._string(),
                '!' => {
                    if self._match_char('=') {
                        self._add_token(TokenType::BANGEQUAL);
                    } else {
                        self._add_token(TokenType::BANG);
                    }
                }
                '=' => {
                    if self._match_char('=') {
                        self._add_token(TokenType::EQUALEQUAL);
                    } else {
                        self._add_token(TokenType::EQUAL);
                    }
                }
                '<' => {
                    if self._match_char('=') {
                        self._add_token(TokenType::LESSEQUAL);
                    } else {
                        self._add_token(TokenType::LESS);
                    }
                }
                '>' => {
                    if self._match_char('=') {
                        self._add_token(TokenType::GREATEREQUAL);
                    } else {
                        self._add_token(TokenType::GREATER);
                    }
                }
                '/' => {
                    if self._match_char('/') {
                        // A comment goes until the end of the line.
                        while self._peek() != Some('\n') && !self.is_at_end() {
                            self._advance();
                        }
                    } else {
                        self._add_token(TokenType::SLASH);
                    }
                }
                ' ' | '\r' | '\t' => {
                    // Ignore whitespace.
                }
                '\n' => {
                    self.line += 1;
                }
                _ => {
                    if c.is_numeric() {
                        self._number();
                    } else if c.is_alphabetic() {
                        self._identifier();
                    } else {
                        Logger::report(
                            self.line,
                            &self.current.to_string(),
                            &format!("Unexpected character: {}", c),
                        );
                        return Some(true);
                    }
                }
            }
        }
        return Some(false);
    }

    fn add_token(&mut self, token_type: TokenType, literal: LiteralType) {
        let sub_string = self.source[self.start..self.current].to_owned();
        self.tokens.push(Token::build(
            token_type,
            &sub_string,
            literal,
            self.line,
            self.current,
        ));
    }
    fn _identifier(&mut self) {
        while let Some(ch) = self._peek() {
            if ch.is_alphanumeric() {
                self._advance();
            } else {
                break;
            }
        }
        let text = self.source[self.start..self.current].to_owned();
        match KEYWORDS.lock().unwrap().get(&text) {
            Some(t) => self.add_token(t.clone(), LiteralType::String(text)),
            None => self.add_token(TokenType::IDENTIFIER, LiteralType::String(text)),
        }
    }
    fn _number(&mut self) {
        while let Some(ch) = self._peek() {
            if ch.is_numeric() {
                self._advance();
            } else {
                break;
            }
        }
        // Look for a fractional part.
        let ch = self._peek().unwrap();
        let chnext = self._peek_next().unwrap();
        if ch == '.' && chnext.is_numeric() {
            // Consume the "."
            self._advance();
            while let Some(ch) = self._peek() {
                if ch.is_numeric() {
                    self._advance();
                } else {
                    break;
                }
            }
        }
        let var_number = self.source[self.start..self.current].parse::<f64>().unwrap();
        self.add_token(TokenType::NUMBER, LiteralType::Number(var_number));
    }

    fn _string(&mut self) {
        while let Some(ch) = self._peek() {
            if ch != '"' && !self.is_at_end() {
                if ch == '\n' {
                    self.line += 1;
                }
                self._advance();
            } else {
                break;
            }
        }
        if self.is_at_end() {
            Logger::error(self.line, "Unterminated string.");
            return;
        }
        // The closing ".
        self._advance();
        // Trim the surrounding quotes.
        let var_string = self.source[self.start + 1..self.current - 1].to_owned();
        self.add_token(TokenType::STRING, LiteralType::String(var_string));
    }
    fn _add_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, LiteralType::Nil);
    }
    fn _match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }
        self.current += 1;
        return true;
    }
    fn _peek(&self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        }
        self.source.chars().nth(self.current)
    }
    fn _peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return Some('\0');
        }
        self.source.chars().nth(self.current + 1)
    }
    fn _advance(&mut self) -> Option<char> {
        // 获取最近的字符
        let c: Option<char> = self.source.chars().nth(self.current);
        self.current += 1;
        c
    }
}



#[cfg(test)]
mod tests_4_scanner {
    use crate::core::scanner::Scanner;
    use crate::core::scanner::TokenType;

    #[test]
    fn test1() {
        let source: String = "(1 + 2) * (4 - 3);".to_string();
        let mut scanner = Scanner::build(&source);
        let tokens = scanner.scan_tokens().clone();

        assert_eq!(tokens.len(), 13);
        assert_eq!(tokens[0]._type, TokenType::LEFTPAREN);
        assert_eq!(tokens[1]._type, TokenType::NUMBER);
        assert_eq!(tokens[2]._type, TokenType::PLUS);
        assert_eq!(tokens[3]._type, TokenType::NUMBER);
        assert_eq!(tokens[4]._type, TokenType::RIGHTPAREN);
        assert_eq!(tokens[5]._type, TokenType::STAR);
        assert_eq!(tokens[6]._type, TokenType::LEFTPAREN);
        assert_eq!(tokens[7]._type, TokenType::NUMBER);
        assert_eq!(tokens[8]._type, TokenType::MINUS);
        assert_eq!(tokens[9]._type, TokenType::NUMBER);
        assert_eq!(tokens[10]._type, TokenType::RIGHTPAREN);
        assert_eq!(tokens[11]._type, TokenType::SEMICOLON);
        assert_eq!(tokens[12]._type, TokenType::EOF);
    }
    
    #[test]
    fn test2() {
        let source: String = "1 >= 2 and 4 < 3;".to_string();
        let mut scanner = Scanner::build(&source);
        let tokens = scanner.scan_tokens().clone();

        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0]._type, TokenType::NUMBER);
        assert_eq!(tokens[1]._type, TokenType::GREATEREQUAL);
        assert_eq!(tokens[2]._type, TokenType::NUMBER);
        assert_eq!(tokens[3]._type, TokenType::AND);
        assert_eq!(tokens[4]._type, TokenType::NUMBER);
        assert_eq!(tokens[5]._type, TokenType::LESS);
        assert_eq!(tokens[6]._type, TokenType::NUMBER);
        assert_eq!(tokens[7]._type, TokenType::SEMICOLON);
        assert_eq!(tokens[8]._type, TokenType::EOF);
    }

    #[test]
    fn test3() {
        let source: String = "1 >= 99 + 5.2 or 2.2 < 3.3;".to_string();
        let mut scanner = Scanner::build(&source);
        let tokens = scanner.scan_tokens().clone();

        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0]._type, TokenType::NUMBER);
        assert_eq!(tokens[1]._type, TokenType::GREATEREQUAL);
        assert_eq!(tokens[2]._type, TokenType::NUMBER);
        assert_eq!(tokens[3]._type, TokenType::PLUS);
        assert_eq!(tokens[4]._type, TokenType::NUMBER);
        assert_eq!(tokens[5]._type, TokenType::OR);
        assert_eq!(tokens[6]._type, TokenType::NUMBER);
        assert_eq!(tokens[7]._type, TokenType::LESS);
        assert_eq!(tokens[8]._type, TokenType::NUMBER);
        assert_eq!(tokens[9]._type, TokenType::SEMICOLON);
        assert_eq!(tokens[10]._type, TokenType::EOF);
    }

}
