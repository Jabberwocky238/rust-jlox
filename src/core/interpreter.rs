use std::rc::Rc;

use super::exprs::Expr;
use super::exprs::Expr::Visitable as _;
use super::exprs::Stmt;
use super::exprs::Stmt::Visitable as _;
use super::scanner::TokenType;
use super::scanner::LiteralType;
use super::scanner::Token;
use super::utils::errors::RuntimeError;

pub struct Interpreter;

impl Expr::Visitor<LiteralType> for Interpreter {
    fn visit_binary_expr(&self, expr: &Expr::Binary) -> LiteralType {
        let left = self.evaluate(&expr.left);
        let right = self.evaluate(&expr.right);

        match expr.operator._type {
            TokenType::GREATER => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                return LiteralType::Boolean(left > right);
            }
            TokenType::GREATEREQUAL => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                return LiteralType::Boolean(left >= right);
            }
            TokenType::LESS => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                return LiteralType::Boolean(left < right);
            }
            TokenType::LESSEQUAL => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                return LiteralType::Boolean(left <= right);
            }
            TokenType::BANGEQUAL => {
                return LiteralType::Boolean(!is_equal(&left, &right));
            }
            TokenType::EQUALEQUAL => {
                return LiteralType::Boolean(is_equal(&left, &right));
            }
            TokenType::MINUS => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                return LiteralType::Number(left - right);
            }
            TokenType::PLUS => {
                if let (LiteralType::Number(left), LiteralType::Number(right)) = (&left, &right) {
                    return LiteralType::Number(*left + *right);
                }
                if let (LiteralType::String(left), LiteralType::String(right)) = (&left, &right) {
                    return LiteralType::String(left.to_string() + &right.to_string());
                }
                panic!("Operands must be two numbers or two strings.");
            }
            TokenType::SLASH => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                return LiteralType::Number(left / right);
            }
            TokenType::STAR => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                return LiteralType::Number(left * right);
            }
            _ => {
                panic!("Unknown operator.");
            }
        }
        // Unreachable.
    }
    fn visit_grouping_expr(&self, expr: &Expr::Group) -> LiteralType {
        return self.evaluate(&expr.expression);
    }
    fn visit_literal_expr(&self, expr: &Expr::Literal) -> LiteralType {
        return expr.value.clone();
    }
    fn visit_unary_expr(&self, expr: &Expr::Unary) -> LiteralType {
        let right = self.evaluate(&expr.right);
        match expr.operator._type {
            TokenType::BANG => {
                let result = !is_truthy(&right);
                return LiteralType::Boolean(result);
            }
            TokenType::MINUS => {
                let right = check_number_operand(&expr.operator, &right).unwrap();
                return LiteralType::Number(-right);
            }
            _ => {
                panic!("Unknown operator.");
            }
        } 
        // Unreachable
    }
    fn visit_assign_expr(&self, expr: &Expr::Assign) -> LiteralType {
        todo!()
    }
    fn visit_variable_expr(&self, expr: &Expr::Variable) -> LiteralType {
        todo!()
    }
}

impl Stmt::Visitor<LiteralType> for Interpreter {
    fn visit_print_stmt(&self, stmt: &Stmt::Print) -> LiteralType {
        let value = self.evaluate(&stmt.expression);
        println!("{value}");
        return LiteralType::Nil;
    }
    fn visit_expression_stmt(&self, stmt: &Stmt::Expression) -> LiteralType {
        self.evaluate(&stmt.expression);
        return LiteralType::Nil;
    }
    fn visit_block_stmt(&self, stmt: &Stmt::Block) -> LiteralType {
        todo!()
    }
    fn visit_var_stmt(&self, stmt: &Stmt::Var) -> LiteralType {
        todo!()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }
    pub fn interpret(&self, stmts: &Vec<Rc<Stmt::Enum>>) -> Result<(), RuntimeError> {
        for stmt in stmts.iter() {
            self.execute(stmt);
        }
        Ok(())
    }
    fn evaluate(&self, expr: &Rc<Expr::Enum>) -> LiteralType {
        return expr.accept(self);
    }
    fn execute(&self, stmt: &Rc<Stmt::Enum>) {
        stmt.accept(self);
    }
}


fn error(token: &Token, message: &String) -> RuntimeError {
    let err = format!("Error {}: {}", token.line, message);
    return RuntimeError(err);
}

fn check_number_operands(operator: &Token, left: &LiteralType, right: &LiteralType) -> Result<(f64, f64), RuntimeError> {
    if let &LiteralType::Number(l) = left {
        if let &LiteralType::Number(r) = right {
            return Ok((l, r));
        }
    }
    let err = format!("Operands must be numbers. Got {}, {}", left, right);
    Err(error(operator, &err))
}

fn check_number_operand(operator: &Token, operand: &LiteralType) -> Result<f64, RuntimeError> {
    if let &LiteralType::Number(v) = operand {
        return Ok(v);
    }
    let err = format!("Operand must be a number. Got {}", operand);
    Err(error(operator, &err))
}

fn is_truthy(object: &LiteralType) -> bool {
    if object == &LiteralType::Nil {
        return false;
    }
    if let &LiteralType::Boolean(value) = object {
        return value;
    }
    return true;
}

fn is_equal(a: &LiteralType, b: &LiteralType) -> bool {
    if &LiteralType::Nil == a && &LiteralType::Nil == b {
        return true;
    }
    if &LiteralType::Nil == a {
        return false;
    }
    return a == b;
}

fn stringify(object: LiteralType) -> String {
    if object == LiteralType::Nil {
        return "nil".to_owned();
    }
    if let LiteralType::Number(value) = object {
        let mut text = value.to_string();
        if text.ends_with(".0") {
            text = text[0..text.len() - 2].to_string();
        }
        return text;
    }
    return object.to_string();
}


#[cfg(test)]
mod tests_4_interpreter {
    use crate::core::interpreter::Interpreter;
    use crate::core::parser::Parser;
    use crate::core::scanner::Scanner;

    fn easy_test(source: &String) -> String {
        let mut scanner = Scanner::build(source);
        let tokens = scanner.scan_tokens().clone();
        let parser = Parser::new(tokens);
        let expression = parser.parse();
        let interpreter = Interpreter::new();
        interpreter.interpret(&expression);
    }

    #[test]
    fn test1() {
        let source: String = "1 >= 99 and 5.2 == 5.2 or 2.2 > 3.3)".to_string();
        let output = easy_test(&source);
        assert_eq!("false", output.as_str());
    }
    
    #[test]
    fn test2() {
        let source: String = "1 >= 99 or 5.2 == 5.2 and 2.2 < 3.3".to_string();
        let output = easy_test(&source);
        assert_eq!("false", output.as_str());
    }
    #[test]
    fn test3() {
        let source: String = "1 + 2 * 3 + 4".to_string();
        let output = easy_test(&source);
        assert_eq!("11", output.as_str());
    }
    #[test]
    fn test4() {
        let source: String = "\"hello\" + \"world\"".to_string();
        let output = easy_test(&source);
        assert_eq!("helloworld", output.as_str());
    }
}
