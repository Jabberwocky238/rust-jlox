use std::rc::Rc;

use crate::core::{exprs::{Expr::{self, Visitable as _}, Stmt::{self, Visitable as _}}, scanner::LiteralType};

pub struct AstPrinter;

impl Expr::Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Expr::Binary) -> String {
        return self.parenthesize(
            &expr.operator.lexeme,
            &[&expr.left, &expr.right],
        );
    }
    fn visit_grouping_expr(&self, expr: &Expr::Group) -> String {
        return self.parenthesize("group", &[&expr.expression]);
    }
    fn visit_literal_expr(&self, expr: &Expr::Literal) -> String {
        stringify(&expr.value)
    }
    fn visit_unary_expr(&self, expr: &Expr::Unary) -> String {
        return self.parenthesize(&expr.operator.lexeme, &[&expr.right]);
    }
    fn visit_assign_expr(&self, expr: &Expr::Assign) -> String {
        todo!()
    }
    fn visit_variable_expr(&self, expr: &Expr::Variable) -> String {
        todo!()
    }
}

impl Stmt::Visitor<String> for AstPrinter {
    fn visit_print_stmt(&self, stmt: &Stmt::Print) -> String {
        return self._stmt_print(&stmt.expression);
    }
    fn visit_expression_stmt(&self, stmt: &Stmt::Expression) -> String {
        return self._stmt_expr(&stmt.expression);
    }
    fn visit_block_stmt(&self, stmt: &Stmt::Block) -> String {
        todo!()
    }
    fn visit_var_stmt(&self, stmt: &Stmt::Var) -> String {
        todo!()
    }
}

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }
    pub fn print_expr(&self, expr: &Rc<Expr::Enum>) -> String {
        return expr.accept(self);
    }
    pub fn print_stmt(&self, stmt: &Rc<Stmt::Enum>) -> String {
        return stmt.accept(self);
    }
    fn parenthesize(&self, name: &str, exprs: &[&Expr::Enum]) -> String {
        let mut string_builder: Vec<String> = vec![];
        string_builder.push("(".to_owned());
        string_builder.push(name.to_owned());
        exprs.iter().for_each(|expr| {
            string_builder.push(" ".to_owned());
            let expr = expr.accept(self);
            string_builder.push(expr);
        });
        string_builder.push(")".to_owned());
        return string_builder.join("");
    }
    fn _stmt_print(&self, expr: &Expr::Enum) -> String {
        let mut string_builder: Vec<&str> = vec![];
        string_builder.push("( print ( ");
        let expr = expr.accept(self);
        string_builder.push(&expr);
        string_builder.push(" ) );");
        return string_builder.join("");
    }
    fn _stmt_expr(&self, expr: &Expr::Enum) -> String {
        let mut string_builder: Vec<&str> = vec![];
        string_builder.push("( ");
        let expr = expr.accept(self);
        string_builder.push(&expr);
        string_builder.push(" );");
        return string_builder.join("");
    }
}

#[allow(unreachable_patterns)]
fn stringify(object: &LiteralType) -> String {
    match object {
        LiteralType::Number(value) => value.to_string(),
        LiteralType::String(value) => ["\"", &value.clone(), "\""].join(""),
        LiteralType::Boolean(value) => ["bool", &value.to_string()].join(" "),
        LiteralType::Nil => "nil".to_owned(),
        _ => object.to_string(),
    }
}

#[cfg(test)]
mod tests_4_ast_printer {
    use std::rc::Rc;
    use crate::core::exprs::Expr;
    use crate::core::interpreter::AstPrinter;
    use crate::core::scanner::{LiteralType, Token, TokenType};

    fn easy_number(num: f64) -> Rc<Expr::Enum> {
        Expr::Literal::build(LiteralType::Number(num))
    }
    fn easy_token(_type: TokenType, lexeme: &str) -> Token {
        Token {
            _type,
            lexeme: lexeme.to_owned(),
            literal: LiteralType::Nil,
            line: 1,
            offset: 1,
        }
    }

    #[test]
    fn test1() {
        let minus = easy_token(TokenType::MINUS, "-");
        let multi = easy_token(TokenType::STAR, "*");
        let _123 = easy_number(123.0);
        let _45dot67 = easy_number(45.67);
        
        let expression = Expr::Binary::build(
            Expr::Unary::build(minus, _123),
            multi,
            Expr::Group::build(_45dot67),
        );

        let ast_parser = AstPrinter::new();
        let output = ast_parser.print(&expression);

        assert_eq!("(* (- 123) (group 45.67))", output.as_str());
    }
}
