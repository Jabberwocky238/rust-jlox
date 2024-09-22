mod expr;
mod stmt;

#[allow(non_snake_case)]
pub mod Expr {
    pub use super::expr::Expr as Enum;
    pub use super::expr::Binary;
    pub use super::expr::Unary;
    pub use super::expr::Group;
    pub use super::expr::Literal;
    pub use super::expr::Assign;
    pub use super::expr::Variable;

    pub trait Visitor<R> {
        fn visit_binary_expr(&self, expr: &Binary) -> R;
        fn visit_grouping_expr(&self, expr: &Group) -> R;
        fn visit_literal_expr(&self, expr: &Literal) -> R;
        fn visit_unary_expr(&self, expr: &Unary) -> R;
        fn visit_assign_expr(&self, expr: &Assign) -> R;
        fn visit_variable_expr(&self, expr: &Variable) -> R;
    }
    pub trait Visitable<R: ?Sized> {
        fn accept(&self, visitor: &dyn Visitor<R>) -> R;
    }
}

#[allow(non_snake_case)]
pub mod Stmt {
    pub use super::stmt::Stmt as Enum;
    pub use super::stmt::Block;
    pub use super::stmt::Expression;
    pub use super::stmt::Print;
    pub use super::stmt::Var;

    pub trait Visitor<R> {
        fn visit_block_stmt(&self, stmt: &Block) -> R;
        fn visit_expression_stmt(&self, stmt: &Expression) -> R;
        fn visit_print_stmt(&self, stmt: &Print) -> R;
        fn visit_var_stmt(&self, stmt: &Var) -> R;
    }
    pub trait Visitable<R: ?Sized> {
        fn accept(&self, visitor: &dyn Visitor<R>) -> R;
    }
}