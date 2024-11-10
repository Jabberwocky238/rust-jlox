use crate::scanner::LiteralType;
use crate::scanner::Token;
use std::rc::Rc;
use paste::paste;

macro_rules! impl_build {
    ($namespace:ty, $token:ty, [ $($param:ident: $t:ty), * ] ) => {
        paste! {
            pub struct $token {
                $( pub $param: $t, )*
            }
            impl $token {
                pub fn build($( $param: $t, )*) -> Rc<$namespace> {
                    let this = Self { $( $param , )* };
                    let warp = $namespace::[< $token >](this);
                    Rc::new(warp)
                }
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

impl_build!( Expr, Binary, [ left: Rc<Expr>, operator: Token, right: Rc<Expr> ] );
impl_build!( Expr, Group, [ expression: Rc<Expr> ] );
impl_build!( Expr, Unary, [ operator: Token, right: Rc<Expr> ] );
impl_build!( Expr, Literal, [ value: LiteralType ] );
impl_build!( Expr, Variable, [ name: Token ] );
impl_build!( Expr, Assign, [ name: Token, value: Rc<Expr> ] );

impl_build!( Stmt, Expression, [ expression: Rc<Expr> ] );
impl_build!( Stmt, Print, [ expression: Rc<Expr> ] );
impl_build!( Stmt, Var, [ name: Token, initializer: Option<Rc<Expr>> ] );
impl_build!( Stmt, Block, [ statements: Vec<Rc<Stmt>> ] );

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            LiteralType::Number(n) => write!(f, "( number {} )", n),
            LiteralType::String(s) => write!(f, "( str \"{}\" )", s),
            LiteralType::Boolean(b) => write!(f, "( bool {} )", b),
            LiteralType::Nil => write!(f, "( nil )"),
        }
    }
}

pub enum Expr {
    Binary(Binary),
    Group(Group),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
    Assign(Assign),
}

pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
    Block(Block),
}

pub trait ExprVisitor<R> {
    fn visit_binary(&self, expr: &Binary) -> R;
    fn visit_grouping(&self, expr: &Group) -> R;
    fn visit_literal(&self, expr: &Literal) -> R;
    fn visit_unary(&self, expr: &Unary) -> R;
    fn visit_variable(&self, expr: &Variable) -> R;
    fn visit_assign(&self, stmt: &Assign) -> R;
}

pub trait StmtVisitor<R> {
    fn visit_expression(&self, stmt: &Expression) -> R;
    fn visit_print(&self, stmt: &Print) -> R;
    fn visit_var(&self, stmt: &Var) -> R;
    fn visit_block(&self, stmt: &Block) -> R;
}

pub trait ExprVisitable<R: ?Sized> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> R;
}

pub trait StmtVisitable<R: ?Sized> {
    fn accept(&self, visitor: &dyn StmtVisitor<R>) -> R;
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
                fn accept(&self, visitor: &dyn ExprVisitor<$output>) -> $output {
                    match self {
                        $(
                            Expr::$op(value) => {
                                visitor.[<visit_ $name>](value)
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
                fn accept(&self, visitor: &dyn StmtVisitor<$output>) -> $output {
                    match self {
                        $(
                            Stmt::$op(value) => {
                                visitor.[<visit_ $name>](value)
                            }
                        )*
                    }
                }
            }
        }
    };
}
