use std::cell::Cell;
use std::rc::Rc;
use std::vec;

use crate::core::exprs::{Expr, Stmt};
use crate::core::scanner::{LiteralType, Token, TokenType};
use crate::core::utils::errors::ParseError;

pub struct Parser {
    current: Cell<usize>,
    tokens: Vec<Token>,
}

fn error(token: Option<&Token>, message: String) -> ParseError {
    let err = format!("Error {}: {}", token.unwrap().line, message);
    return ParseError(err);
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            current: Cell::new(0),
            tokens,
        }
    }
    pub fn parse(&self) -> Vec<Rc<Stmt::Enum>> {
        // self.expression()
        let mut statements = vec![];
        while !self.is_at_end() {
            let stmt = self.statement().unwrap();
            statements.push(stmt);
        }
        return statements;
    }
    fn statement(&self) -> Result<Rc<Stmt::Enum>, ParseError> {
        if self._match(&[TokenType::PRINT]) {
            return self.print_statement();
        }
        return self.expression_statement();
    }

    fn print_statement(&self) -> Result<Rc<Stmt::Enum>, ParseError> {
        let expr = self.expression().unwrap();
        self.consume(&TokenType::SEMICOLON, "Expect ';' after value.".to_string()).unwrap();
        let stmt = Stmt::Print::build(expr);
        return Ok(stmt);
    }

    fn expression_statement(&self) -> Result<Rc<Stmt::Enum>, ParseError> {
        let expr = self.expression().unwrap();
        let _ = self.consume(
            &TokenType::SEMICOLON,
            "Expect ';' after expression.".to_string(),
        );
        let stmt = Stmt::Expression::build(expr);
        return Ok(stmt);
    }
    fn expression(&self) -> Result<Rc<Expr::Enum>, ParseError> {
        return self.equality();
    }

    fn equality(&self) -> Result<Rc<Expr::Enum>, ParseError> {
        let mut expr = self.comparison().unwrap();

        while self._match(&[TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
            let operator = self.previous().unwrap();
            let right = self.comparison().unwrap();
            expr = Expr::Binary::build(expr, operator.clone(), right);
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

    fn comparison(&self) -> Result<Rc<Expr::Enum>, ParseError> {
        let mut expr = self.term().unwrap();
        while self._match(&[
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ]) {
            let operator = self.previous().unwrap();
            let right = self.term().unwrap();
            expr = Expr::Binary::build(expr, operator.clone(), right);
        }
        return Ok(expr);
    }

    fn term(&self) -> Result<Rc<Expr::Enum>, ParseError> {
        let mut expr = self.factor().unwrap();
        while self._match(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().unwrap();
            let right = self.factor().unwrap();
            expr = Expr::Binary::build(expr, operator.clone(), right);
        }
        return Ok(expr);
    }

    fn factor(&self) -> Result<Rc<Expr::Enum>, ParseError> {
        let mut expr = self.unary().unwrap();
        while self._match(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().unwrap();
            let right = self.unary().unwrap();
            expr = Expr::Binary::build(expr, operator.clone(), right);
        }
        return Ok(expr);
    }

    fn unary(&self) -> Result<Rc<Expr::Enum>, ParseError> {
        if self._match(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().unwrap();
            let right = self.unary().unwrap();
            return Ok(Expr::Unary::build(operator.clone(), right));
        }
        return self.primary();
    }

    fn primary(&self) -> Result<Rc<Expr::Enum>, ParseError> {
        if self._match(&[TokenType::FALSE]) {
            return Ok(Expr::Literal::build(LiteralType::Boolean(false)));
        }
        if self._match(&[TokenType::TRUE]) {
            return Ok(Expr::Literal::build(LiteralType::Boolean(true)));
        }
        if self._match(&[TokenType::NIL]) {
            return Ok(Expr::Literal::build(LiteralType::Nil));
        }
        if self._match(&[TokenType::NUMBER, TokenType::STRING]) {
            if let Some(token) = self.previous() {
                return Ok(Expr::Literal::build(token.literal.clone()));
            }
        }
        if self._match(&[TokenType::LEFTPAREN]) {
            let expr = self.expression().unwrap();
            let _ = self.consume(
                &TokenType::RIGHTPAREN,
                String::from("Expect ')' after expression."),
            );
            return Ok(Expr::Group::build(expr));
        }
        Err(error(self.peek(), String::from("Expect expression.")))
    }

    fn consume(&self, _type: &TokenType, message: String) -> Result<(), ParseError> {
        if self.check(_type) {
            self.advance();
            return Ok(());
        }
        Err(error(self.peek(), message))
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
    use crate::core::parser::AstPrinter;
    use crate::core::parser::Parser;
    use crate::core::scanner::Scanner;

    fn easy_test(source: String) -> String {
        let mut scanner = Scanner::build(&source);
        let tokens = scanner.scan_tokens().clone();
        let parser = Parser::new(tokens);
        let expression = parser.parse().unwrap();
        let ast_parser = AstPrinter::new();
        ast_parser.print(&expression)
    }

    #[test]
    fn test1() {
        let source: String = "(1 + 2) * (4 - 3);".to_string();
        let output = easy_test(source);

        assert_eq!("(* (group (+ 1 2)) (group (- 4 3)))", output.as_str());
    }

    #[test]
    fn test2() {
        let source: String = "1 >= 99 + 5.2 == 2.2 > 3.3;".to_string();
        let output = easy_test(source);

        assert_eq!("(== (>= 1 (+ 99 5.2)) (> 2.2 3.3))", output.as_str());
    }
}
