use std::cell::Cell;
use std::vec;

use crate::ast::*;

use crate::token::TokenLiteral;
use crate::token::Token;
use crate::token::TokenType;

use crate::errors::ParseError;

pub struct Parser {
    current: Cell<usize>,
    tokens: Vec<Token>,
}

type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            current: Cell::new(0),
            tokens,
        }
    }
    pub fn parse(self) -> ParseResult<Vec<RcStmt>> {
        let mut stmts = vec![];
        while !self._is_end() {
            let stmt = self.declaration()?;
            stmts.push(stmt);
        }
        return Ok(stmts);
    }

    fn declaration(&self) -> ParseResult<RcStmt> {
        if self._match(&[TokenType::FUN]) {
            return self.function("function");
        }
        if self._match(&[TokenType::VAR]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn function(&self, kind: &str) -> ParseResult<RcStmt> {
        let name = self._consume(&TokenType::IDENTIFIER, &format!("Expect {} name.", kind))?;
        self._consume(&TokenType::LEFTPAREN, &format!("Expect '(' after {} name.", kind))?;
        let mut params = vec![];
        if !self._check(&TokenType::RIGHTPAREN) {
            loop {
                if params.len() >= 255 {
                    return Err(ParseError::new(self._peek(), "Cannot have more than 255 parameters."));
                }
                let t = self._consume(&TokenType::IDENTIFIER, "Expect parameter name.")?;
                params.push(t.clone());
                if !self._match(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        self._consume(&TokenType::RIGHTPAREN, "Expect ')' after parameters.")?;
        self._consume(&TokenType::LEFTBRACE, &format!("Expect '{{' before {} body.", kind))?;
        let body = self.block()?;
        return Ok(Function::build(name.clone(), params, body));
    }

    fn var_declaration(&self) -> ParseResult<RcStmt> {
        let name = self._consume(&TokenType::IDENTIFIER, "Expect variable name.")?;
        let initializer = if self._match(&[TokenType::EQUAL]) {
            Some(self.expression()?)
        } else {
            None
        };
        self._consume(
            &TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;
        return Ok(Var::build(name.clone(), initializer));
    }

    fn statement(&self) -> ParseResult<RcStmt> {
        if self._match(&[TokenType::FOR]) {
            return self.for_statement();
        }
        if self._match(&[TokenType::IF]) {
            return self.if_statement();
        }
        if self._match(&[TokenType::PRINT]) {
            return self.print_statement();
        }
        if self._match(&[TokenType::RETURN]) {
            return self.return_statement();
        }
        if self._match(&[TokenType::WHILE]) {
            return self.while_statement();
        }
        if self._match(&[TokenType::LEFTBRACE]) {
            return self.block();
        }
        return self.expression_statement();
    }

    fn for_statement(&self) -> ParseResult<RcStmt> {
        self._consume(&TokenType::LEFTPAREN, "Expect '(' after 'for'.")?;

        let initializer = if self._match(&[TokenType::SEMICOLON]) {
            None
        } else if self._match(&[TokenType::VAR]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self._check(&TokenType::SEMICOLON) {
            Some(self.expression()?)
        } else {
            None
        };
        self._consume(&TokenType::SEMICOLON, "Expect ';' after loop condition.")?;

        let increment = if !self._check(&TokenType::RIGHTPAREN) {
            Some(self.expression()?)
        } else {
            None
        };
        self._consume(&TokenType::RIGHTPAREN, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;
        
        if let Some(increment) = increment {
            body = Block::build(vec![body, Expression::build(increment)]);
        }
        if let Some(condition) = condition {
            body = While::build(condition, body);
        } else {
            body = While::build(Literal::build(TokenLiteral::Bool(true)), body);
        }
        if let Some(initializer) = initializer {
            body = Block::build(vec![initializer, body]);
        }
        
        return Ok(body);
    }

    fn if_statement(&self) -> ParseResult<RcStmt> {
        self._consume(&TokenType::LEFTPAREN, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self._consume(&TokenType::RIGHTPAREN, "Expect ')' after if condition.")?;
        let then_branch = self.statement()?;
        let else_branch = if self._match(&[TokenType::ELSE]) {
            Some(self.statement()?)
        } else {
            None
        };
        return Ok(If::build(condition, then_branch, else_branch));
    }

    fn print_statement(&self) -> ParseResult<RcStmt> {
        let value = self.expression()?;
        self._consume(&TokenType::SEMICOLON, "Expect ';' after value.")?;
        return Ok(Print::build(value));
    }

    fn return_statement(&self) -> ParseResult<RcStmt> {
        let keyword = self._previous().unwrap();
        let value = if !self._check(&TokenType::SEMICOLON) {
            Some(self.expression()?)
        } else {
            None
        };
        self._consume(&TokenType::SEMICOLON, "Expect ';' after return value.")?;
        return Ok(Return::build(keyword.clone(), value));
    }
    
    fn while_statement(&self) -> ParseResult<RcStmt> {
        self._consume(&TokenType::LEFTPAREN, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self._consume(&TokenType::RIGHTPAREN, "Expect ')' after condition.")?;
        let body = self.statement()?;
        return Ok(While::build(condition, body));
    }


    fn block(&self) -> ParseResult<RcStmt> {
        let mut statements = vec![];
        while !self._check(&TokenType::RIGHTBRACE) && !self._is_end() {
            statements.push(self.declaration()?);
        }
        self._consume(&TokenType::RIGHTBRACE, "Expect '}' after block.")?;
        return Ok(Block::build(statements));
    }

    fn expression_statement(&self) -> ParseResult<RcStmt> {
        let value = self.expression()?;
        self._consume(&TokenType::SEMICOLON, "Expect ';' after value.")?;
        return Ok(Expression::build(value));
    }

    fn expression(&self) -> ParseResult<RcExpr> {
        return self.assignment();
    }

    fn assignment(&self) -> ParseResult<RcExpr> {
        let expr = self.or()?;
        if self._match(&[TokenType::EQUAL]) {
            let equals = self._previous().unwrap();
            let value = self.assignment()?;
            if let Expr::Variable(x) = expr.as_ref() {
                let name = x.name.clone();
                return Ok(Assign::build(name, value));
            }
            return Err(ParseError::new(Some(equals), "Invalid assignment target."));
        }
        return Ok(expr);
    }

    fn or(&self) -> ParseResult<RcExpr> {
        let mut expr = self.and()?;
        while self._match(&[TokenType::OR]) {
            let operator = self._previous().unwrap();
            let right = self.and()?;
            expr = Logical::build(expr, operator.clone(), right);
        }
        return Ok(expr);
    }

    fn and(&self) -> ParseResult<RcExpr> {
        let mut expr = self.equality()?;
        while self._match(&[TokenType::AND]) {
            let operator = self._previous().unwrap();
            let right = self.equality()?;
            expr = Logical::build(expr, operator.clone(), right);
        }
        return Ok(expr);
    }

    fn equality(&self) -> ParseResult<RcExpr> {
        let mut expr = self.comparison().unwrap();

        while self._match(&[TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
            let operator = self._previous().unwrap();
            let right = self.comparison().unwrap();
            expr = Binary::build(expr, operator.clone(), right);
        }

        return Ok(expr);
    }

    fn comparison(&self) -> ParseResult<RcExpr> {
        let mut expr = self.term().unwrap();
        while self._match(&[
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ]) {
            let operator = self._previous().unwrap();
            let right = self.term().unwrap();
            expr = Binary::build(expr, operator.clone(), right);
        }
        return Ok(expr);
    }

    fn term(&self) -> ParseResult<RcExpr> {
        let mut expr = self.factor().unwrap();
        while self._match(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self._previous().unwrap();
            let right = self.factor().unwrap();
            expr = Binary::build(expr, operator.clone(), right);
        }
        return Ok(expr);
    }

    fn factor(&self) -> ParseResult<RcExpr> {
        let mut expr = self.unary().unwrap();
        while self._match(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self._previous().unwrap();
            let right = self.unary().unwrap();
            expr = Binary::build(expr, operator.clone(), right);
        }
        return Ok(expr);
    }

    fn unary(&self) -> ParseResult<RcExpr> {
        if self._match(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self._previous().unwrap();
            let right = self.unary().unwrap();
            return Ok(Unary::build(operator.clone(), right));
        }
        return self.call();
    }

    fn call(&self) -> ParseResult<RcExpr> {
        let mut expr = self.primary().unwrap();
        loop {
            if self._match(&[TokenType::LEFTPAREN]) {
                expr = self.finish_call(expr).unwrap();
            } else {
                break;
            }
        }
        return Ok(expr);
    }

    fn finish_call(&self, callee: RcExpr) -> ParseResult<RcExpr> {
        let mut arguments = vec![];
        if !self._check(&TokenType::RIGHTPAREN) {
            loop {
                if arguments.len() >= 255 {
                    return Err(ParseError::new(self._peek(), "Cannot have more than 255 arguments."));
                }
                arguments.push(self.expression().unwrap());
                if !self._match(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        let paren = self._consume(&TokenType::RIGHTPAREN, "Expect ')' after arguments.")?;
        return Ok(Call::build(callee, paren.clone(), arguments));
    }

    fn primary(&self) -> ParseResult<RcExpr> {
        if self._match(&[TokenType::FALSE]) {
            return Ok(Literal::build(TokenLiteral::Bool(false)));
        }
        if self._match(&[TokenType::TRUE]) {
            return Ok(Literal::build(TokenLiteral::Bool(true)));
        }
        if self._match(&[TokenType::NIL]) {
            return Ok(Literal::build(TokenLiteral::Nil));
        }
        if self._match(&[TokenType::NUMBER, TokenType::STRING]) {
            if let Some(token) = self._previous() {
                return Ok(Literal::build(token.literal.clone()));
            }
        }
        if self._match(&[TokenType::IDENTIFIER]) {
            if let Some(token) = self._previous() {
                return Ok(Variable::build(token.clone()));
            }
        }
        if self._match(&[TokenType::LEFTPAREN]) {
            let expr = self.expression()?;
            let _ = self._consume(&TokenType::RIGHTPAREN, "Expect ')' after expression.");
            return Ok(Group::build(expr));
        }
        Err(ParseError::new(self._peek(), "Expect expression."))
    }

    fn _match(&self, types: &[TokenType]) -> bool {
        for _type in types {
            if self._check(_type) {
                self._advance();
                return true;
            }
        }
        return false;
    }

    fn _check(&self, _type: &TokenType) -> bool {
        if self._is_end() {
            return false;
        }
        if let Some(token) = self._peek() {
            return &token._type == _type;
        }
        return false;
    }

    fn _advance(&self) -> Option<&Token> {
        if !self._is_end() {
            self.current.set(self.current.get() + 1);
        }
        return self._previous();
    }

    fn _is_end(&self) -> bool {
        if let Some(token) = self._peek() {
            token._type == TokenType::EOF
        } else {
            false
        }
    }

    fn _peek(&self) -> Option<&Token> {
        self.tokens.get(self.current.get())
    }

    fn _previous(&self) -> Option<&Token> {
        self.tokens.get(self.current.get() - 1)
    }

    fn _consume(&self, _type: &TokenType, message: &str) -> Result<&Token, ParseError> {
        if self._check(_type) {
            if let Some(token) = self._advance() {
                return Ok(token);
            }
        }
        Err(ParseError::new(self._peek(), message))
    }

    fn synchronize(&mut self) {
        self._advance();
        while !self._is_end() {
            if let Some(current) = self._previous() {
                if current._type == TokenType::SEMICOLON {
                    return;
                }
            }
            if let Some(token) = self._peek() {
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
            self._advance();
        }
    }
}
