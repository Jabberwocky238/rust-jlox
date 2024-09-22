use crate::core::scanner::LiteralType;
use crate::core::scanner::Token;
use std::rc::Rc;

use super::Expr::Visitable;
use super::Expr::Visitor;

pub enum Expr {
    Binary(Binary),
    Group(Group),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
    Assign(Assign),
}

// astprinter
impl Visitable<String> for Expr {
    fn accept(&self, visitor: &dyn Visitor<String>) -> String {
        match self {
            Expr::Binary(value) => {
                visitor.visit_binary_expr(value)
            }
            Expr::Group(value) => {
                visitor.visit_grouping_expr(value)
            }
            Expr::Literal(value) => {
                visitor.visit_literal_expr(value)
            }
            Expr::Unary(value) => {
                visitor.visit_unary_expr(value)
            },
            Expr::Variable(variable) => {
                visitor.visit_variable_expr(variable)
            },
            Expr::Assign(assign) => {
                visitor.visit_assign_expr(assign)
            },
        }
    }
}

// interpreter
impl Visitable<LiteralType> for Expr {
    fn accept(&self, visitor: &dyn Visitor<LiteralType>) -> LiteralType {
        match self {
            Expr::Binary(value) => {
                visitor.visit_binary_expr(value)
            }
            Expr::Group(value) => {
                visitor.visit_grouping_expr(value)
            }
            Expr::Literal(value) => {
                visitor.visit_literal_expr(value)
            }
            Expr::Unary(value) => {
                visitor.visit_unary_expr(value)
            },
            Expr::Variable(variable) => {
                visitor.visit_variable_expr(variable)
            },
            Expr::Assign(assign) => {
                visitor.visit_assign_expr(assign)
            },
        }
    }
}

// 二元表达式
pub struct Binary {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}
// 括号表达式
pub struct Group {
    pub expression: Rc<Expr>,
}
// 一元表达式
pub struct Unary {
    pub operator: Token,
    pub right: Rc<Expr>,
}
// 字面量表达式
pub struct Literal {
    pub value: LiteralType,
}

// 变量表达式
pub struct Variable {
    pub name: Token,
}

// 赋值表达式
pub struct Assign {
    pub name: Token,
    pub value: Rc<Expr>,
}

impl Binary {
    pub fn build(left: Rc<Expr>, operator: Token, right: Rc<Expr>) -> Rc<Expr> {
        let this = Self { left, operator, right };
        let warp = Expr::Binary(this);
        Rc::new(warp)
    }
}

impl Group {
    pub fn build(expression: Rc<Expr>) -> Rc<Expr> {
        let this = Self { expression };
        let warp = Expr::Group(this);
        Rc::new(warp)
    }
}

impl Unary {
    pub fn build(operator: Token, right: Rc<Expr>) -> Rc<Expr> {
        let this = Self { operator, right };
        let warp = Expr::Unary(this);
        Rc::new(warp)
    }
}

impl Literal {
    pub fn build(value: LiteralType) -> Rc<Expr> {
        let this = Self { value };
        let warp = Expr::Literal(this);
        Rc::new(warp)
    }
}


