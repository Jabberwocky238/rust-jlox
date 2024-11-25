use std::cell::Cell;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::ast;
use crate::ast::*;
use crate::interpreter::Interpreter;
use crate::token::Token;
use ast::{ExprVisitable, ExprVisitor, RcExpr, RcStmt, StmtVisitable, StmtVisitor};

use crate::impl_expr_visitable;
use crate::impl_stmt_visitable;

impl_expr_visitable! {
    <()>,
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
    <()>,
    (Expression, expression),
    (Print, print),
    (Var, var),
    (Block, block),
    (If, if),
    (While, while),
    (Function, function),
    (Return, return),
}

pub struct ScopeResolver<'a> {
    pub scopes: RefCell<Vec<HashMap<String, bool>>>,
    interpreter: RefCell<&'a mut Interpreter>,
}

impl<'a> ScopeResolver<'a>
where
    Self: ExprVisitor<()> + StmtVisitor<()>,
{
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            scopes: RefCell::new(Vec::new()),
            interpreter: interpreter.into(),
        }
    }
    pub fn resolve(&self, statements: &Vec<RcStmt>) {
        for stmt in statements {
            self.resolve_stmt(stmt.clone());
        }
    }
    fn begin_scope(&self) {
        self.scopes.borrow_mut().push(HashMap::new());
    }
    fn end_scope(&self) {
        self.scopes.borrow_mut().pop();
    }
    fn resolve_local(&self, expr: RcExpr, name: Token) {
        let borrowed_scope = self.scopes.borrow();
        for (i, scope) in borrowed_scope.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.borrow_mut().resolve(expr, borrowed_scope.len() - 1 - i);
                return;
            }
        }
    }
    fn resolve_function(&self, func: &Function) {
        self.begin_scope();
        for param in func.params.iter() {
            self.declare(&param);
            self.define(&param);
        }
        self.resolve_stmt(func.body.clone());
        self.end_scope();
    }
    fn resolve_stmt(&self, stmt: RcStmt) {
        <ast::Stmt as Clone>::clone(&stmt).accept(self);
    }
    fn resolve_expr(&self, expr: RcExpr) {
        <ast::Expr as Clone>::clone(&expr).accept(self);
    }
    fn declare(&self, token: &Token) {
        if self.scopes.borrow().is_empty() {
            return;
        }
        if let Some(scope) = self.scopes.borrow_mut().last_mut() {
            scope.insert(token.lexeme.clone(), false);
        }
    }
    fn define(&self, token: &Token) {
        if self.scopes.borrow().is_empty() {
            return;
        }
        if let Some(scope) = self.scopes.borrow_mut().last_mut() {
            scope.insert(token.lexeme.clone(), true);
        }
    }
}

impl ExprVisitor<()> for ScopeResolver<'_> {
    fn visit_binary(&self, expr: &ast::Binary) {
        self.resolve_expr(expr.left.clone());
        self.resolve_expr(expr.right.clone());
    }

    fn visit_grouping(&self, expr: &ast::Group) {
        self.resolve_expr(expr.expression.clone());
    }

    fn visit_literal(&self, _: &ast::Literal) {}

    fn visit_unary(&self, expr: &ast::Unary) {
        self.resolve_expr(expr.right.clone());
    }

    fn visit_variable(&self, expr: &ast::Variable) {
        let borrowed_scope = self.scopes.borrow();
        if !borrowed_scope.is_empty() {
            if let Some(scope) = borrowed_scope.last() {
                if let Some(false) = scope.get(&expr.name.lexeme) {
                    panic!("Can't read local variable in its own initializer.")
                }
            }
        }
        self.resolve_local(Rc::new(ast::Expr::Variable(expr.clone())), expr.name.clone());
    }

    fn visit_assign(&self, expr: &ast::Assign) {
        self.resolve_expr(expr.value.clone());
        self.resolve_local(Rc::new(ast::Expr::Assign(expr.clone())), expr.name.clone());
    }

    fn visit_logical(&self, expr: &ast::Logical) {
        self.resolve_expr(expr.left.clone());
        self.resolve_expr(expr.right.clone());
    }

    fn visit_call(&self, expr: &ast::Call) {
        self.resolve_expr(expr.callee.clone());
        for arg in expr.arguments.iter() {
            self.resolve_expr(arg.clone());
        }
    }
}

impl StmtVisitor<()> for ScopeResolver<'_> {
    fn visit_expression(&self, stmt: &ast::Expression) {
        self.resolve_expr(stmt.expression.clone());
    }

    fn visit_print(&self, stmt: &ast::Print) {
        self.resolve_expr(stmt.expression.clone());
    }

    fn visit_var(&self, stmt: &ast::Var) {
        self.declare(&stmt.name);
        // declare(stmt.name);
        if let Some(initializer) = &stmt.initializer {
            self.resolve_expr(initializer.clone());
        }
        self.define(&stmt.name);
    }

    fn visit_block(&self, stmt: &ast::Block) {
        self.begin_scope();
        self.resolve(&stmt.statements);
        self.end_scope();
    }

    fn visit_if(&self, stmt: &ast::If) {
        self.resolve_expr(stmt.condition.clone());
        self.resolve_stmt(stmt.then_branch.clone());
        if let Some(else_branch) = &stmt.else_branch {
            self.resolve_stmt(else_branch.clone());
        }
    }

    fn visit_while(&self, stmt: &ast::While) {
        self.resolve_expr(stmt.condition.clone());
        self.resolve_stmt(stmt.body.clone());
    }

    fn visit_function(&self, stmt: &ast::Function) {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.resolve_function(stmt);
    }

    fn visit_return(&self, stmt: &ast::Return) {
        if let Some(value) = &stmt.value {
            self.resolve_expr(value.clone());
        }
    }
}

// ----------------------------------------------------------------
