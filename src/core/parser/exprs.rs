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

use std::rc::Rc;

use crate::core::scanner::Token;
use crate::core::scanner::LiteralType;
use super::exprvisiter::ExprVisitor;

// 表达式的抽象
pub trait ExprT {
    fn accept(&self, visitor: &dyn ExprVisitor<String>) -> String;
}

pub type Expr = Rc<dyn ExprT>;

// 二元表达式
pub struct Binary {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

impl ExprT for Binary {
    fn accept(&self, visitor: &dyn ExprVisitor<String>) -> String {
        visitor.visit_binary_expr(self)
    }
}

impl Binary {
    pub fn build(left: Expr, operator: Token, right: Expr) -> Rc<Binary> {
        Rc::new(Self { left, operator, right })
    }
}

// 括号表达式
pub struct Group {
    pub expression: Expr,
}

impl ExprT for Group {
    fn accept(&self, visitor: &dyn ExprVisitor<String>) -> String {
        visitor.visit_grouping_expr(self)
    }
}

impl Group {
    pub fn build(expression: Expr) -> Rc<Group> {
        Rc::new(Self { expression })
    }
}


// 一元表达式
pub struct Unary {
    pub operator: Token,
    pub right: Expr,
}

impl ExprT for Unary {
    fn accept(&self, visitor: &dyn ExprVisitor<String>) -> String {
        visitor.visit_unary_expr(self)
    }
}

impl Unary {
    pub fn build(operator: Token, right: Expr) -> Rc<Unary> {
        Rc::new(Self { operator, right })
    }
}

// 字面量表达式
pub struct Literal {
    pub value: LiteralType
}

impl ExprT for Literal {
    fn accept(&self, visitor: &dyn ExprVisitor<String>) -> String {
        visitor.visit_literal_expr(self)
    }
}

impl Literal {
    pub fn build(value: LiteralType) -> Rc<Literal> {
        Rc::new(Self { value })
    }
}
