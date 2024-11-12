use std::cell::RefCell;
use std::rc::Rc;

use super::ast::*;

use super::token::LoxLiteral;
use super::token::TokenType;
use super::token::LoxValue;
use super::token::Token;
use super::errors::RuntimeError;

// impl Visitable<LiteralType> for Expr {
//     fn accept(&self, visitor: &dyn Visitor<LiteralType>) -> LiteralType {
//         match self {
//             Expr::Binary(value) => {
//                 visitor.visit_binary_expr(value)
//             }
//             Expr::Group(value) => {
//                 visitor.visit_grouping_expr(value)
//             }
//             Expr::Literal(value) => {
//                 visitor.visit_literal_expr(value)
//             }
//             Expr::Unary(value) => {
//                 visitor.visit_unary_expr(value)
//             },
//         }
//     }
// }

use crate::environment::Environment;
use crate::impl_expr_visitable;
use crate::impl_stmt_visitable;

impl_expr_visitable! {
    <LoxValue>, 
    (Binary, binary),
    (Group, grouping),
    (Literal, literal),
    (Unary, unary),
    (Variable, variable),
    (Assign, assign),
    (Logical, logical),
    (Call, call),
}

type RuntimeResult = Result<(), RuntimeError>;
impl_stmt_visitable! {
    <RuntimeResult>, 
    (Expression, expression),
    (Print, print),
    (Var, var),
    (Block, block),
    (If, if),
    (While, while),
    (Function, function),
    (Return, return),
}

pub struct Interpreter{
    pub environment: RefCell<Environment>,
}

impl Interpreter where Self: ExprVisitor<LoxValue> {
    pub fn new() -> Self {
        Interpreter {
            environment: RefCell::new(Environment::new()),
        }
    }
    pub fn interpret(&self, stmts: &Vec<Rc<Stmt>>) -> RuntimeResult {
        for stmt in stmts.iter() {
            self.execute(stmt)?;
        }
        Ok(())
    }
    fn execute(&self, stmt: &Rc<Stmt>) -> RuntimeResult {
        stmt.accept(self)
    }
    fn evaluate(&self, expr: &Rc<Expr>) -> LoxValue {
        return expr.accept(self);
    }
}

impl ExprVisitor<LoxValue> for Interpreter {
    fn visit_binary(&self, expr: &Binary) -> LoxValue {
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
    fn visit_grouping(&self, expr: &Group) -> LoxValue {
        return self.evaluate(&expr.expression);
    }
    fn visit_literal(&self, expr: &Literal) -> LoxValue {
        return expr.value.clone();
    }
    fn visit_unary(&self, expr: &Unary) -> LoxValue {
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
    
    fn visit_variable(&self, expr: &Variable) -> LiteralType {
        return self.environment.borrow().get(&expr.name.lexeme).unwrap();
    }
    
    fn visit_assign(&self, stmt: &Assign) -> LiteralType {
        let value = self.evaluate(&stmt.value);
        self.environment.borrow_mut().assign(&stmt.name.lexeme, value.clone()).unwrap();
        return value;
    }
    
    fn visit_logical(&self, stmt: &Logical) -> LiteralType {
        let left = self.evaluate(&stmt.left);
        if stmt.operator._type == TokenType::OR {
            if is_truthy(&left) {
                return left;
            }
        } else {
            if !is_truthy(&left) {
                return left;
            }
        }
        return self.evaluate(&stmt.right);
    }
    
    fn visit_call(&self, stmt: &Call) -> LoxValue {
        todo!()
    }
}

impl StmtVisitor<RuntimeResult> for Interpreter {
    fn visit_expression(&self, stmt: &Expression) -> RuntimeResult {
        let _ = self.evaluate(&stmt.expression);
        Ok(())
    }
    fn visit_print(&self, stmt: &Print) -> RuntimeResult {
        let value = self.evaluate(&stmt.expression);
        println!("{}", value);
        Ok(())
    }
    fn visit_var(&self, stmt: &Var) -> RuntimeResult {
        let value = if let Some(ref initializer) = stmt.initializer {
            self.evaluate(initializer)
        } else {
            LoxValue::Literal(LoxLiteral::Nil)
        };
        self.environment.borrow_mut().define(&stmt.name.lexeme, value);
        Ok(())
    }
    
    fn visit_block(&self, stmt: &Block) -> RuntimeResult {
        self.environment.borrow_mut().enter_scope();
            
        for statement in stmt.statements.iter() {
            self.execute(statement)?;
        }

        self.environment.borrow_mut().exit_scope();
        Ok(())
    }
    
    fn visit_if(&self, stmt: &If) -> RuntimeResult {
        if is_truthy(&self.evaluate(&stmt.condition)) {
            self.execute(&stmt.then_branch)?;
        } else if let Some(ref else_branch) = stmt.else_branch {
            self.execute(else_branch)?;
        }
        Ok(())
    }
    
    fn visit_while(&self, stmt: &While) -> RuntimeResult {
        while is_truthy(&self.evaluate(&stmt.condition)) {
            self.execute(&stmt.body)?;
        }
        Ok(())
    }
    
    fn visit_function(&self, stmt: &Function) -> RuntimeResult {
        todo!()
    }
    
    fn visit_return(&self, stmt: &Return) -> RuntimeResult {
        todo!()
    }
}

// ----------------------------------------------------------------
// ----------------------------------------------------------------

fn check_number_operands(operator: &Token, left: &LiteralType, right: &LiteralType) -> Result<(f64, f64), RuntimeError> {
    if let &LiteralType::Number(l) = left {
        if let &LiteralType::Number(r) = right {
            return Ok((l, r));
        }
    }
    let err = format!("Operands must be numbers. Got {}, {}", left, right);
    Err(RuntimeError::new(operator, &err))
}

fn check_number_operand(operator: &Token, operand: &LiteralType) -> Result<f64, RuntimeError> {
    if let &LiteralType::Number(v) = operand {
        return Ok(v);
    }
    let err = format!("Operand must be a number. Got {}", operand);
    Err(RuntimeError::new(operator, &err))
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

// fn stringify(object: LiteralType) -> String {
//     if object == LiteralType::Nil {
//         return "nil".to_owned();
//     }
//     if let LiteralType::Number(value) = object {
//         let mut text = value.to_string();
//         if text.ends_with(".0") {
//             text = text[0..text.len() - 2].to_string();
//         }
//         return text;
//     }
//     return object.to_string();
// }
