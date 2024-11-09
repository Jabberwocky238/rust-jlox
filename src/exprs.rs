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

use crate::scanner::LiteralType;
use crate::scanner::Token;
use std::rc::Rc;
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

pub enum Expr {
    Binary(Binary),
    Group(Group),
    Literal(Literal),
    Unary(Unary),
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

pub trait Visitor<R> {
    fn visit_binary_expr(&self, expr: &Binary) -> R;
    fn visit_grouping_expr(&self, expr: &Group) -> R;
    fn visit_literal_expr(&self, expr: &Literal) -> R;
    fn visit_unary_expr(&self, expr: &Unary) -> R;
}
pub trait Visitable<R: ?Sized> {
    fn accept(&self, visitor: &dyn Visitor<R>) -> R;
}

#[macro_export]
macro_rules! impl_visitable {
    {   
        impl <$output:ty> for $enum:ty, 
        $(
            ( $op:ident, $name:ident ),
        )*
    } => {
        use paste::paste;
        paste! {
            impl Visitable<$output> for $enum {
                fn accept(&self, visitor: &dyn Visitor<$output>) -> $output {
                    match self {
                        $(
                            $enum::$op(value) => {
                                visitor.[<visit_ $name _expr>](value)
                            }
                        )*
                    }
                }
            }
        }
    };
}


//     fn accept(&self, visitor: &dyn Visitor<String>) -> String {
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