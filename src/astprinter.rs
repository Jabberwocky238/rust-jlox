use crate::ast;
use crate::ast::*;

use crate::impl_expr_visitable;
use crate::impl_stmt_visitable;

pub struct AstPrinter;

impl_expr_visitable! {
    <String>, 
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
    <String>, 
    (Expression, expression),
    (Print, print),
    (Var, var),
    (Block, block),
    (If, if),
    (While, while),
    (Function, function),
    (Return, return),
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&self, expr: &Binary) -> String {
        return self.parenthesize(
            &expr.operator.lexeme,
            &[&expr.left, &expr.right],
        );
    }
    fn visit_grouping(&self, expr: &Group) -> String {
        return self.parenthesize("group", &[&expr.expression]);
    }
    fn visit_literal(&self, expr: &Literal) -> String {
        expr.value.to_string()
    }
    fn visit_unary(&self, expr: &Unary) -> String {
        return self.parenthesize(&expr.operator.lexeme, &[&expr.right]);
    }
    
    fn visit_variable(&self, expr: &Variable) -> String {
        String::from(expr.name.lexeme.clone())
    }
    
    fn visit_assign(&self, stmt: &Assign) -> String {
        let value = <ast::Expr as Clone>::clone(&stmt.value).accept(self);
        String::from(format!("{} = {}", stmt.name.lexeme, value))
    }
    
    fn visit_logical(&self, stmt: &Logical) -> String {
        return self.parenthesize(&stmt.operator.lexeme, &[&stmt.left, &stmt.right]);
    }
    
    fn visit_call(&self, stmt: &Call) -> String {
        todo!()
    }
}

impl StmtVisitor<String> for AstPrinter {
    fn visit_expression(&self, stmt: &Expression) -> String {
        let value = <ast::Expr as Clone>::clone(&stmt.expression).accept(self);
        return format!("( {} )", value);
    }
    fn visit_print(&self, stmt: &Print) -> String {
        return self.parenthesize(&"print", &[&stmt.expression]);
    }
    fn visit_var(&self, stmt: &Var) -> String {
        match &stmt.initializer {
            Some(expr) => {
                let value = <ast::Expr as Clone>::clone(&expr).accept(self);
                return format!("( var {} = {} )", stmt.name.lexeme, value);
            },
            None => {
                return format!("( var {} )", stmt.name.lexeme);
            },
        }
    }
    fn visit_block(&self, stmt: &Block) -> String {
        let mut string_builder: Vec<String> = vec![];
        string_builder.push("{".to_owned());
        stmt.statements.iter().for_each(|stmt| {
            string_builder.push("\n".to_owned());
            let stmt = <ast::Stmt as Clone>::clone(&stmt).accept(self);
            string_builder.push(stmt);
        });
        string_builder.push("\n}".to_owned());
        return string_builder.join("");
    }
    
    fn visit_if(&self, stmt: &If) -> String {
        let mut string_builder: Vec<String> = vec![];
        string_builder.push("if ".to_owned());
        string_builder.push(<ast::Expr as Clone>::clone(&stmt.condition).accept(self));
        string_builder.push(" ( ".to_owned());
        string_builder.push(<ast::Stmt as Clone>::clone(&stmt.then_branch).accept(self));
        string_builder.push(" ) ".to_owned());
        if let Some(else_branch) = &stmt.else_branch {
            string_builder.push(" else ( ".to_owned());
            string_builder.push(<ast::Stmt as Clone>::clone(&else_branch).accept(self));
            string_builder.push(" ) ".to_owned());
        }
        string_builder.push("\n".to_owned());
        return string_builder.join("");
    }
    
    fn visit_while(&self, stmt: &While) -> String {
        let mut string_builder: Vec<String> = vec![];
        string_builder.push("( while ".to_owned());
        string_builder.push(<ast::Expr as Clone>::clone(&stmt.condition).accept(self));
        string_builder.push(" (".to_owned());
        string_builder.push(<ast::Stmt as Clone>::clone(&stmt.body).accept(self));
        string_builder.push(")\n".to_owned());
        return string_builder.join("");
    }
    
    fn visit_function(&self, stmt: &Function) -> String {
        todo!()
    }
    
    fn visit_return(&self, stmt: &Return) -> String {
        todo!()
    }
}

impl AstPrinter 
where Self: ExprVisitor<String> 
{
    pub fn new() -> Self {
        AstPrinter {}
    }
    pub fn print_expr(&self, expr: RcExpr) -> String {
        <ast::Expr as Clone>::clone(&expr).accept(self)
    }
    pub fn print_stmt(&self, stmt: RcStmt) -> String {
        <ast::Stmt as Clone>::clone(&stmt).accept(self)
    }
    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut string_builder: Vec<String> = vec![];
        string_builder.push("(".to_owned());
        string_builder.push(name.to_owned());
        exprs.iter().for_each(|expr| {
            string_builder.push(" ".to_owned());
            let expr = <ast::Expr as Clone>::clone(&expr).accept(self);
            string_builder.push(expr);
        });
        string_builder.push(")".to_owned());
        return string_builder.join("");
    }
}


