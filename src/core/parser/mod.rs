use std::cell::Cell;

use exprs::Binary;
use exprs::Expr;
use exprs::Group;
use exprs::Literal;
use exprs::Unary;

use crate::core::scanner::LiteralType;
use crate::core::scanner::TokenType;

use super::scanner::Token;

// expression     → literal
//                | unary
//                | binary
//                | grouping ;
// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;

pub mod exprs;
pub mod exprvisiter;

pub struct Parser {
    current: Cell<usize>,
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: Cell::new(0), tokens }
    }
    pub fn parse(&self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&self) -> Result<Expr, ParseError> {
        return self.equality();
    }

    fn equality(&self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison().unwrap();

        while self._match(&[TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
            let operator = self.previous().unwrap();
            let right = self.comparison().unwrap();
            expr = Binary::build(expr, operator.clone(), right);
        }

        return Ok(expr);
    }

    fn _match(&self, types: &[TokenType]) -> bool {
        for _type in types {
            if self.check(_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, _type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Some(token) = self.peek() {
            return &token._type == _type;
        }
        return false;
    }

    fn advance(&self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current.set(self.current.get() + 1);
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        if let Some(token) = self.peek() {
            return token._type == TokenType::EOF;
        }
        return false;
    }

    fn peek(&self) -> Option<&Token> {
        return self.tokens.get(self.current.get());
    }

    fn previous(&self) -> Option<&Token> {
        return self.tokens.get(self.current.get() - 1);
    }

    fn comparison(&self) -> Result<Expr, ParseError> {
        let mut expr = self.term().unwrap();

        while self._match(&[
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ]) {
            let operator = self.previous().unwrap();
            let right = self.term().unwrap();
            expr = Binary::build(expr, operator.clone(), right);
        }

        return Ok(expr);
    }

    fn term(&self) -> Result<Expr, ParseError> {
        let mut expr = self.factor().unwrap();

        while self._match(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().unwrap();
            let right = self.factor().unwrap();
            expr = Binary::build(expr, operator.clone(), right);
        }

        return Ok(expr);
    }

    fn factor(&self) -> Result<Expr, ParseError> {
        let mut expr = self.unary().unwrap();

        while self._match(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().unwrap();
            let right = self.unary().unwrap();
            expr = Binary::build(expr, operator.clone(), right);
        }

        return Ok(expr);
    }

    fn unary(&self) -> Result<Expr, ParseError> {
        if self._match(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().unwrap();
            let right = self.unary().unwrap();
            return Ok(Unary::build(operator.clone(), right));
        }
        return self.primary();
    }

    fn primary(&self) -> Result<Expr, ParseError> {
        if self._match(&[TokenType::FALSE]) {
            return Ok(Literal::build(LiteralType::Boolean(false)));
        }
        if self._match(&[TokenType::TRUE]) {
            return Ok(Literal::build(LiteralType::Boolean(true)));
        }
        if self._match(&[TokenType::NIL]) {
            return Ok(Literal::build(LiteralType::Nil));
        }
        if self._match(&[TokenType::NUMBER, TokenType::STRING]) {
            if let Some(token) = self.previous() {
                return Ok(Literal::build(token.literal.clone()));
            }
        }
        if self._match(&[TokenType::LEFTPAREN]) {
            let expr = self.expression().unwrap();
            let _ = self.consume(
                &TokenType::RIGHTPAREN,
                String::from("Expect ')' after expression."),
            );
            return Ok(Group::build(expr));
        }
        Err(self.error(self.peek(), String::from("Expect expression.")))
    }

    fn consume(&self, _type: &TokenType, message: String) -> Result<(), ParseError> {
        if self.check(_type) {
            self.advance();
            return Ok(());
        }
        Err(self.error(self.peek(), message))
    }

    fn error(&self, token: Option<&Token>, message: String) -> ParseError {
        return ParseError { 
            message: format!("Error {}: {}", token.unwrap().line, message)
        };
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let Some(current) = self.previous() {
                if current._type == TokenType::SEMICOLON {
                    return;
                }
            }

            if let Some(token) = self.peek() {
                match token._type {
                    TokenType::CLASS
                    | TokenType::FUN
                    | TokenType::VAR
                    | TokenType::FOR
                    | TokenType::IF
                    | TokenType::WHILE
                    | TokenType::PRINT
                    | TokenType::RETURN => return,
                    _ => (),
                }
            }
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests_4_parser {
    use crate::core::parser::exprvisiter::AstPrinter;
    use crate::core::parser::Parser;
    use crate::core::scanner::Scanner;

    #[test]
    fn test1() {
        let source: String = "(1 + 2) * (4 - 3);".to_string();
        let mut scanner = Scanner::build(&source);
        let tokens = scanner.scan_tokens().clone();
        let parser = Parser::new(tokens);
        let expression = parser.parse().unwrap();
        let ast_parser = AstPrinter::new();
        let output = ast_parser.print(expression);

        assert_eq!("(* (group (+ 1 2)) (group (- 4 3)))", output.as_str());
    }

    // #[test]
    // fn test2() {
    //     let source: String = "1 >= 99 + 5.2 or 2.2 < 3.3;".to_string();
    //     let mut scanner = Scanner::build(&source);
    //     let tokens = scanner.scan_tokens().clone();
    //     let parser = Parser::new(tokens);
    //     let expression = parser.parse().unwrap();
    //     let ast_parser = AstPrinter::new();
    //     let output = ast_parser.print(expression);

    //     assert_eq!("(or (>= 1 (+ 99 5.2)) (< 2.2 3.3))", output.as_str());
    // }

    #[test]
    fn test() {
        let source: String = "1 >= 99 + 5.2 == 2.2 > 3.3;".to_string();
        let mut scanner = Scanner::build(&source);
        let tokens = scanner.scan_tokens().clone();
        let parser = Parser::new(tokens);
        let expression = parser.parse().unwrap();
        let ast_parser = AstPrinter::new();
        let output = ast_parser.print(expression);

        assert_eq!("(== (>= 1 (+ 99 5.2)) (> 2.2 3.3))", output.as_str());
    }
}
