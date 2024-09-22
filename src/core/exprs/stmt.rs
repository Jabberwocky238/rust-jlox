use std::rc::Rc;
use crate::core::scanner::{LiteralType, Token};
use super::expr::Expr;
use super::Stmt::{Visitable, Visitor};

pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
    Block(Block),
}

// astprinter
impl Visitable<String> for Stmt {
    fn accept(&self, visitor: &dyn Visitor<String>) -> String {
        match self {
            Stmt::Expression(expression) => {
                visitor.visit_expression_stmt(expression)
            },
            Stmt::Print(print) => {
                visitor.visit_print_stmt(print)
            },
            Stmt::Var(var) => {
                visitor.visit_var_stmt(var)
            },
            Stmt::Block(block) => {
                visitor.visit_block_stmt(block)
            },
        }
    }
}

// interpreter
impl Visitable<LiteralType> for Stmt {
    fn accept(&self, visitor: &dyn Visitor<LiteralType>) -> LiteralType {
        match self {
            Stmt::Expression(expression) => {
                visitor.visit_expression_stmt(expression)
            },
            Stmt::Print(print) => {
                visitor.visit_print_stmt(print)
            },
            Stmt::Var(var) => {
                visitor.visit_var_stmt(var)
            },
            Stmt::Block(block) => {
                visitor.visit_block_stmt(block)
            },
        }
    }
}

pub struct Block {
    pub statements: Vec<Rc<Stmt>>,
}

pub struct Expression {
    pub expression: Rc<Expr>,
}

pub struct Print {
    pub expression: Rc<Expr>,
}

pub struct Var {
    pub name: Token,
    pub initializer: Rc<Expr>,
}

impl Block {
    pub fn build(statements: Vec<Rc<Stmt>>) -> Rc<Stmt> {
        let this = Self { statements };
        let warp = Stmt::Block(this);
        Rc::new(warp)
    }
}

impl Expression {
    pub fn build(expression: Rc<Expr>) -> Rc<Stmt> {
        let this = Self { expression };
        let warp = Stmt::Expression(this);
        Rc::new(warp)
    }
}

impl Print {
    pub fn build(expression: Rc<Expr>) -> Rc<Stmt> {
        let this = Self { expression };
        let warp = Stmt::Print(this);
        Rc::new(warp)
    }
}

impl Var {
    pub fn build(name: Token, initializer: Rc<Expr>) -> Rc<Stmt> {
        let this = Self { name, initializer };
        let warp = Stmt::Var(this);
        Rc::new(warp)
    }
}

