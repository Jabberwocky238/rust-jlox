use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::FlatMap;
use std::rc::Rc;

use super::ast::*;

use crate::ast;
use crate::environment::Environment;

use crate::function::builtin_function_clock;
use crate::function::LoxFunction;

use super::token::Token;
use super::token::TokenLiteral;
use super::token::TokenType;

use super::errors::RuntimeError;
use super::errors::RuntimeErrorT;
use super::errors::RuntimeReturn;

type RuntimeResult = Result<(), Box<dyn Any>>;
use crate::impl_expr_visitable;
use crate::impl_stmt_visitable;

impl_expr_visitable! {
    <Rc<LoxValue>>,
    (Binary, binary),
    (Group, grouping),
    (Literal, literal),
    (Unary, unary),
    (Variable, variable),
    (Assign, assign),
    (Logical, logical),
    (Call, call),
}
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

pub struct Interpreter {
    pub environment: RefCell<Environment>,
    pub locals: RefCell<HashMap<RcExpr, usize>>,
}

impl Interpreter
where
    Self: ExprVisitor<Rc<LoxValue>> + StmtVisitor<RuntimeResult>,
{
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.define("clock", builtin_function_clock().into());
        Interpreter {
            environment: RefCell::new(env),
            locals: RefCell::new(HashMap::new()),
        }
    }
    pub fn interpret(&self, stmts: &Vec<RcStmt>) -> RuntimeResult {
        for stmt in stmts {
            self.execute(stmt.clone())?;
        }
        Ok(())
    }
    pub fn resolve(&self, expr: RcExpr, depth: usize) {
        self.locals.borrow_mut().insert(expr, depth);
    }
    fn lookup_variable(&self, name: &Token, expr: &RcExpr) -> Rc<LoxValue> {
        let distance = self.locals.borrow().get(expr);
        if let Some(distance) = distance {
            return self
                .environment
                .borrow()
                .get_at(*distance, &name.lexeme)
                .unwrap();
        } else {
            return self.environment.borrow().get(&name.lexeme).unwrap();
        }
    }
    fn execute(&self, stmt: RcStmt) -> RuntimeResult {
        <ast::Stmt as Clone>::clone(&stmt).accept(self)
    }
    fn evaluate(&self, expr: RcExpr) -> Rc<LoxValue> {
        <ast::Expr as Clone>::clone(&expr).accept(self)
    }
}

impl ExprVisitor<Rc<LoxValue>> for Interpreter {
    fn visit_binary(&self, expr: &Binary) -> Rc<LoxValue> {
        let binding = self.evaluate(expr.left.clone());
        let left = binding.as_ref();
        let binding = self.evaluate(expr.right.clone());
        let right = binding.as_ref();

        let ret = match expr.operator._type {
            TokenType::GREATER => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                LoxValue::Bool(left > right)
            }
            TokenType::GREATEREQUAL => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                LoxValue::Bool(left >= right)
            }
            TokenType::LESS => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                LoxValue::Bool(left < right)
            }
            TokenType::LESSEQUAL => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                LoxValue::Bool(left <= right)
            }
            TokenType::BANGEQUAL => LoxValue::Bool(!is_equal(&left, &right)),
            TokenType::EQUALEQUAL => LoxValue::Bool(is_equal(&left, &right)),
            TokenType::MINUS => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                LoxValue::Number(left - right)
            }
            TokenType::PLUS => {
                if let (LoxValue::Number(left), LoxValue::Number(right)) = (&left, &right) {
                    LoxValue::Number(left + right)
                } else if let (LoxValue::String(left), LoxValue::String(right)) = (&left, &right) {
                    LoxValue::String(left.to_string() + &right.to_string())
                } else {
                    panic!("Operands must be two numbers or two strings.");
                }
            }
            TokenType::SLASH => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                LoxValue::Number(left / right)
            }
            TokenType::STAR => {
                let (left, right) = check_number_operands(&expr.operator, &left, &right).unwrap();
                LoxValue::Number(left * right)
            }
            _ => {
                panic!("Unknown operator.");
            }
        };
        return ret.into();
        // Unreachable.
    }
    fn visit_grouping(&self, expr: &Group) -> Rc<LoxValue> {
        self.evaluate(expr.expression.clone())
    }
    fn visit_literal(&self, expr: &Literal) -> Rc<LoxValue> {
        let ret = match expr.value.clone() {
            TokenLiteral::Number(value) => LoxValue::Number(value),
            TokenLiteral::String(value) => LoxValue::String(value),
            TokenLiteral::Bool(value) => LoxValue::Bool(value),
            TokenLiteral::Nil => LoxValue::Nil,
        };
        return ret.into();
    }
    fn visit_unary(&self, expr: &Unary) -> Rc<LoxValue> {
        let right = self.evaluate(expr.right.clone());
        let ret = match expr.operator._type {
            TokenType::BANG => {
                let result = !is_truthy(&right);
                LoxValue::Bool(result)
            }
            TokenType::MINUS => {
                let right = check_number_operand(&expr.operator, &right).unwrap();
                LoxValue::Number(-right)
            }
            _ => {
                panic!("Unknown operator.");
            }
        };
        return ret.into();
    }

    fn visit_variable(&self, expr: &Variable) -> Rc<LoxValue> {
        // 11.4
        // return self.environment.borrow().get(&expr.name.lexeme).unwrap();
        return self.lookup_variable(&expr.name, expr);
    }

    fn visit_assign(&self, stmt: &Assign) -> Rc<LoxValue> {
        let value = self.evaluate(stmt.value.clone());
        // 11.4
        // self.environment.borrow_mut().assign(&stmt.name.lexeme, value.clone()).unwrap();
        let distance = self.locals.borrow().get(stmt);
        if let Some(distance) = distance {
            self.environment
                .borrow_mut()
                .assign_at(*distance, &stmt.name.lexeme, value.clone())
                .unwrap();
        } else {
            self.environment
                .borrow_mut()
                .assign(&stmt.name.lexeme, value.clone())
                .unwrap();
        }
        return value.clone();
    }

    fn visit_logical(&self, stmt: &Logical) -> Rc<LoxValue> {
        let left = self.evaluate(stmt.left.clone());
        if stmt.operator._type == TokenType::OR {
            if is_truthy(&left) {
                return left;
            }
        } else {
            if !is_truthy(&left) {
                return left;
            }
        }
        return self.evaluate(stmt.right.clone());
    }

    fn visit_call(&self, stmt: &Call) -> Rc<LoxValue> {
        let callee = self.evaluate(stmt.callee.clone());
        let mut arguments = Vec::new();

        for argument in stmt.arguments.clone() {
            let arg = self.evaluate(argument);
            arguments.push(arg);
        }
        if let LoxValue::Callable(callee) = callee.as_ref() {
            return callee.call(self, arguments).into();
        }
        panic!("Can only call functions and classes.");
    }
}

impl StmtVisitor<RuntimeResult> for Interpreter {
    fn visit_expression(&self, stmt: &Expression) -> RuntimeResult {
        let _ = self.evaluate(stmt.expression.clone());
        Ok(())
    }
    fn visit_print(&self, stmt: &Print) -> RuntimeResult {
        let value = self.evaluate(stmt.expression.clone());
        println!("{}", value);
        Ok(())
    }
    fn visit_var(&self, stmt: &Var) -> RuntimeResult {
        let value = if let Some(initializer) = &stmt.initializer {
            self.evaluate(initializer.clone())
        } else {
            Rc::new(LoxValue::Nil)
        };
        self.environment
            .borrow_mut()
            .define(&stmt.name.lexeme, value.into());
        Ok(())
    }

    fn visit_block(&self, stmt: &Block) -> RuntimeResult {
        self.environment.borrow_mut().enter_scope(false);

        for statement in &stmt.statements {
            self.execute(statement.clone())?;
        }

        self.environment.borrow_mut().exit_scope();
        Ok(())
    }

    fn visit_if(&self, stmt: &If) -> RuntimeResult {
        if is_truthy(&self.evaluate(stmt.condition.clone())) {
            self.execute(stmt.then_branch.clone())?;
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch.clone())?;
        }
        Ok(())
    }

    fn visit_while(&self, stmt: &While) -> RuntimeResult {
        while is_truthy(&self.evaluate(stmt.condition.clone())) {
            self.execute(stmt.body.clone())?;
        }
        Ok(())
    }

    fn visit_function(&self, stmt: &Function) -> RuntimeResult {
        let function_name = stmt.name.lexeme.clone();
        let function = LoxValue::Callable(Box::new(LoxFunction::new(stmt.clone())));
        self.environment
            .borrow_mut()
            .define(&function_name, function.into());
        Ok(())
    }

    fn visit_return(&self, stmt: &Return) -> RuntimeResult {
        let ret = if let Some(value) = &stmt.value {
            self.evaluate(value.clone())
        } else {
            Rc::new(LoxValue::Nil)
        };
        Err(Box::new(RuntimeReturn::new(ret)))
    }
}

// ----------------------------------------------------------------
// ----------------------------------------------------------------

fn check_number_operands(
    operator: &Token,
    left: &LoxValue,
    right: &LoxValue,
) -> Result<(f64, f64), RuntimeError> {
    if let (LoxValue::Number(l), LoxValue::Number(r)) = (left, right) {
        return Ok((*l, *r));
    }
    let err = format!("Operands must be numbers. Got {}, {}", left, right);
    Err(RuntimeError::new(operator, &err))
}

fn check_number_operand(operator: &Token, operand: &LoxValue) -> Result<f64, RuntimeError> {
    if let &LoxValue::Number(v) = operand {
        return Ok(v);
    }
    let err = format!("Operand must be a number. Got {}", operand);
    Err(RuntimeError::new(operator, &err))
}

fn is_truthy(object: &LoxValue) -> bool {
    if let LoxValue::Nil = object {
        return false;
    }
    if let LoxValue::Bool(value) = object {
        return *value;
    }
    return true;
}

fn is_equal(a: &LoxValue, b: &LoxValue) -> bool {
    if let (LoxValue::Nil, LoxValue::Nil) = (a, b) {
        return true;
    }
    if let LoxValue::Nil = a {
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
