use std::cell::Cell;
use std::rc::Rc;
use std::vec;

use crate::gen::*;

use crate::scanner::LiteralType;
use crate::scanner::TokenType;
use crate::scanner::Token;

use crate::errors::ParseError;

pub struct Parser {
    current: Cell<usize>,
    tokens: Vec<Token>,
}

fn error(token: Option<&Token>, message: &str) -> ParseError {
    ParseError(
        format!("Error {}: {}", token.unwrap().line, message)
    )
}

type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: Cell::new(0), tokens }
    }
    pub fn parse(&self) -> ParseResult<Vec<Rc<Stmt>>> {
        let mut stmts = vec![];
        while !self.is_at_end() {
            let stmt = self.declaration()?;
            stmts.push(stmt);
        }
        return Ok(stmts);
    }

    fn declaration(&self) -> ParseResult<Rc<Stmt>> {
        if self._match(&[TokenType::VAR]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&self) -> ParseResult<Rc<Stmt>> {
        let name = self.consume(&TokenType::IDENTIFIER, "Expect variable name.")?;
        let initializer = if self._match(&[TokenType::EQUAL]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(&TokenType::SEMICOLON, "Expect ';' after variable declaration.")?;
        return Ok(Var::build(name.clone(), initializer));
    }

    fn statement(&self) -> ParseResult<Rc<Stmt>> {
        if self._match(&[TokenType::PRINT]) {
            return self.print_statement();
        }
        if self._match(&[TokenType::LEFTBRACE]) {
            return self.block();
        }
        return self.expression_statement();
    }

    fn print_statement(&self) -> ParseResult<Rc<Stmt>> {
        let value = self.expression()?;
        self.consume(&TokenType::SEMICOLON, "Expect ';' after value.")?;
        return Ok(Print::build(value));
    }

    fn block(&self) -> ParseResult<Rc<Stmt>> {
        let mut statements = vec![];
        while !self.check(&TokenType::RIGHTBRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(&TokenType::RIGHTBRACE, "Expect '}' after block.")?;
        return Ok(Block::build(statements));
    }

    fn expression_statement(&self) -> ParseResult<Rc<Stmt>> {
        let value = self.expression()?;
        self.consume(&TokenType::SEMICOLON, "Expect ';' after value.")?;
        return Ok(Expression::build(value));
    }

    fn expression(&self) -> Result<Rc<Expr>, ParseError> {
        return self.assignment();
    }

    fn assignment(&self) -> Result<Rc<Expr>, ParseError> {
        let expr = self.equality()?;
        if self._match(&[TokenType::EQUAL]) {
            let equals = self.previous().unwrap();
            let value = self.assignment()?;
            if let Expr::Variable(x) = expr.as_ref() {
                let name = x.name.clone();
                return Ok(Assign::build(name, value));
            }
            return Err(error(Some(equals), "Invalid assignment target."));
        }
        return Ok(expr);
    }


    fn equality(&self) -> Result<Rc<Expr>, ParseError> {
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

    fn comparison(&self) -> Result<Rc<Expr>, ParseError> {
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

    fn term(&self) -> Result<Rc<Expr>, ParseError> {
        let mut expr = self.factor().unwrap();
        while self._match(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().unwrap();
            let right = self.factor().unwrap();
            expr = Binary::build(expr, operator.clone(), right);
        }
        return Ok(expr);
    }

    fn factor(&self) -> Result<Rc<Expr>, ParseError> {
        let mut expr = self.unary().unwrap();
        while self._match(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().unwrap();
            let right = self.unary().unwrap();
            expr = Binary::build(expr, operator.clone(), right);
        }
        return Ok(expr);
    }

    fn unary(&self) -> Result<Rc<Expr>, ParseError> {
        if self._match(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().unwrap();
            let right = self.unary().unwrap();
            return Ok(Unary::build(operator.clone(), right));
        }
        return self.primary();
    }

    fn primary(&self) -> Result<Rc<Expr>, ParseError> {
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
        if self._match(&[TokenType::IDENTIFIER]) {
            if let Some(token) = self.previous() {
                return Ok(Variable::build(token.clone()));
            }
        }
        if self._match(&[TokenType::LEFTPAREN]) {
            let expr = self.expression().unwrap();
            let _ = self.consume(
                &TokenType::RIGHTPAREN,
                "Expect ')' after expression.",
            );
            return Ok(Group::build(expr));
        }
        Err(error(self.peek(), "Expect expression."))
    }

    fn consume(&self, _type: &TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(_type) {
            if let Some(token) = self.advance() {
                return Ok(token);
            }
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

// #[cfg(test)]
// mod tests_4_parser {
//     use crate::astprinter::AstPrinter;
//     use crate::parser::Parser;
//     use crate::scanner::Scanner;

//     fn easy_test(source: String) -> String {
//         let mut scanner = Scanner::build(&source);
//         let tokens = scanner.scan_tokens().clone();
//         let parser = Parser::new(tokens);
//         let expression = parser.parse().unwrap();
//         let ast_parser = AstPrinter::new();
//         ast_parser.print(&expression)
//     }

//     #[test]
//     fn test1() {
//         let source: String = "(1 + 2) * (4 - 3);".to_string();
//         let output = easy_test(source);

//         assert_eq!("(* (group (+ 1 2)) (group (- 4 3)))", output.as_str());
//     }

//     #[test]
//     fn test2() {
//         let source: String = "1 >= 99 + 5.2 == 2.2 > 3.3;".to_string();
//         let output = easy_test(source);

//         assert_eq!("(== (>= 1 (+ 99 5.2)) (> 2.2 3.3))", output.as_str());
//     }
// }
