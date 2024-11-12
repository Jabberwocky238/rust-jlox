use crate::token::LoxLiteral;
use crate::token::Token;
use paste::paste;

macro_rules! impl_build {
    ($namespace:ty, $token:ty, [ $($param:ident: $t:ty), * ] ) => {
        paste! {
            pub struct $token {
                $( pub $param: $t, )*
            }
            impl $token {
                pub fn build($( $param: $t, )*) -> Box<$namespace> {
                    let this = Self { $( $param , )* };
                    let warp = $namespace::[< $token >](this);
                    Box::new(warp)
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

pub type BoxExpr = Box<Expr>;
pub type BoxStmt = Box<Stmt>;

impl_build!( Expr, Binary, [ left: BoxExpr, operator: Token, right: BoxExpr ] );
impl_build!( Expr, Group, [ expression: BoxExpr ] );
impl_build!( Expr, Unary, [ operator: Token, right: BoxExpr ] );
impl_build!( Expr, Literal, [ value: LoxLiteral ] );
impl_build!( Expr, Variable, [ name: Token ] );
impl_build!( Expr, Assign, [ name: Token, value: BoxExpr ] );
impl_build!( Expr, Logical, [ left: BoxExpr, operator: Token, right: BoxExpr ] );
impl_build!( Expr, Call, [ callee: BoxExpr, paren: Token, arguments: Vec<BoxExpr> ] );

impl_build!( Stmt, Expression, [ expression: BoxExpr ] );
impl_build!( Stmt, Print, [ expression: BoxExpr ] );
impl_build!( Stmt, Var, [ name: Token, initializer: Option<BoxExpr> ] );
impl_build!( Stmt, Block, [ statements: Vec<BoxStmt> ] );
impl_build!( Stmt, If, [ condition: BoxExpr, then_branch: BoxStmt, else_branch: Option<BoxStmt> ] );
impl_build!( Stmt, While, [ condition: BoxExpr, body: BoxStmt ] );
impl_build!( Stmt, Function, [ name: Token, params: Vec<Token>, body: BoxStmt ] );
impl_build!( Stmt, Return, [ keyword: Token, value: Option<BoxExpr> ] );

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
