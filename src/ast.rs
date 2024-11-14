use std::fmt::Debug;
use std::rc::Rc;

use crate::function::LoxCallable;
use crate::token::TokenLiteral;
use crate::token::Token;
use paste::paste;

macro_rules! impl_build {
    ($namespace:ty, $token:ty, [ $($param:ident: $t:ty), * ] ) => {
        paste! {
            #[derive(Debug, Clone)]
            pub struct $token {
                $( pub $param: $t, )*
            }
            impl $token {
                pub fn build($( $param: $t, )*) -> Rc<$namespace> {
                    let this = Self { $( $param , )* };
                    let warp = $namespace::[< $token >](this);
                    Rc::new(warp)
                }
                // pub fn build($( $param: $t, )*) -> $namespace {
                //     let this = Self { $( $param , )* };
                //     let warp = $namespace::[< $token >](this);
                //     warp
                // }
            }
        }
    };
}

// 二元表达式
// pub struct Binary {
//     pub left: Rc<Expr>,
//     pub operator: Token,
//     pub right: Rc<Expr>,
// }
// impl Binary {
//     pub fn build(left: Rc<Expr>, operator: Token, right: Rc<Expr>) -> Rc<Expr> {
//         let this = Self { left, operator, right };
//         let warp = Expr::Binary(this);
//         Rc::new(warp)
//     }
// }

pub type RcExpr = Rc<Expr>;
pub type RcStmt = Rc<Stmt>;

impl_build!( Expr, Binary, [ left: RcExpr, operator: Token, right: RcExpr ] );
impl_build!( Expr, Group, [ expression: RcExpr ] );
impl_build!( Expr, Unary, [ operator: Token, right: RcExpr ] );
impl_build!( Expr, Literal, [ value: TokenLiteral ] );
impl_build!( Expr, Variable, [ name: Token ] );
impl_build!( Expr, Assign, [ name: Token, value: RcExpr ] );
impl_build!( Expr, Logical, [ left: RcExpr, operator: Token, right: RcExpr ] );
impl_build!( Expr, Call, [ callee: RcExpr, paren: Token, arguments: Vec<RcExpr> ] );

impl_build!( Stmt, Expression, [ expression: RcExpr ] );
impl_build!( Stmt, Print, [ expression: RcExpr ] );
impl_build!( Stmt, Var, [ name: Token, initializer: Option<RcExpr> ] );
impl_build!( Stmt, Block, [ statements: Vec<RcStmt> ] );
impl_build!( Stmt, If, [ condition: RcExpr, then_branch: RcStmt, else_branch: Option<RcStmt> ] );
impl_build!( Stmt, While, [ condition: RcExpr, body: RcStmt ] );
impl_build!( Stmt, Function, [ name: Token, params: Vec<Token>, body: RcStmt ] );
impl_build!( Stmt, Return, [ keyword: Token, value: Option<RcExpr> ] );

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Group(Group),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
    Block(Block),
    If(If),
    While(While),
    Function(Function),
    Return(Return),
}

pub trait ExprVisitor<R>
{
    fn visit_binary(&self, expr: &Binary) -> R;
    fn visit_grouping(&self, expr: &Group) -> R;
    fn visit_literal(&self, expr: &Literal) -> R;
    fn visit_unary(&self, expr: &Unary) -> R;
    fn visit_variable(&self, expr: &Variable) -> R;
    fn visit_assign(&self, stmt: &Assign) -> R;
    fn visit_logical(&self, stmt: &Logical) -> R;
    fn visit_call(&self, stmt: &Call) -> R;
}

pub trait StmtVisitor<R>
{
    fn visit_expression(&self, stmt: &Expression) -> R;
    fn visit_print(&self, stmt: &Print) -> R;
    fn visit_var(&self, stmt: &Var) -> R;
    fn visit_block(&self, stmt: &Block) -> R;
    fn visit_if(&self, stmt: &If) -> R;
    fn visit_while(&self, stmt: &While) -> R;
    fn visit_function(&self, stmt: &Function) -> R;
    fn visit_return(&self, stmt: &Return) -> R;
}

pub trait ExprVisitable<R: ?Sized> {
    fn accept(self, visitor: &dyn ExprVisitor<R>) -> R;
}

pub trait StmtVisitable<R: ?Sized> {
    fn accept(self, visitor: &dyn StmtVisitor<R>) -> R;
}

#[macro_export]
macro_rules! impl_expr_visitable {
    {
        <$output:ty>,
        $(
            ( $op:ident, $name:ident ),
        )*
    } => {
        paste::paste! {
            impl ExprVisitable<$output> for Expr {
                fn accept(self, visitor: &dyn ExprVisitor<$output>) -> $output {
                    match self {
                        $(
                            Expr::$op(value) => {
                                visitor.[<visit_ $name>](&value)
                            }
                        )*
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_stmt_visitable {
    {
        <$output:ty>,
        $(
            ( $op:ident, $name:ident ),
        )*
    } => {
        paste::paste! {
            impl StmtVisitable<$output> for Stmt {
                fn accept(self, visitor: &dyn StmtVisitor<$output>) -> $output {
                    match self {
                        $(
                            Stmt::$op(value) => {
                                visitor.[<visit_ $name>](&value)
                            }
                        )*
                    }
                }
            }
        }
    };
}


pub enum LoxValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    Callable(Box<dyn for<'a> LoxCallable>),
}

impl Default for LoxValue {
    fn default() -> Self {
        LoxValue::Nil
    }
}

impl PartialEq for LoxValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LoxValue::Number(a), LoxValue::Number(b)) => a == b,
            (LoxValue::String(a), LoxValue::String(b)) => a == b,
            (LoxValue::Bool(a), LoxValue::Bool(b)) => a == b,
            (LoxValue::Nil, LoxValue::Nil) => true,
            _ => false,
        }
    }   
}

impl std::fmt::Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxValue::Number(n) => write!(f, "{}", n),
            LoxValue::String(s) => write!(f, "{}", s),
            LoxValue::Bool(b) => write!(f, "{}", b),
            LoxValue::Nil => write!(f, "nil"),
            // LoxValue::Literal(l) => write!(f, "{}", l),
            LoxValue::Callable(c) => write!(f, "{}", c),
        }
    }
}

impl Debug for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Bool(arg0) => f.debug_tuple("Bool").field(arg0).finish(),
            Self::Nil => write!(f, "Nil"),
            Self::Callable(_arg0) => f.debug_tuple("Callable").finish(),
        }
    }
}