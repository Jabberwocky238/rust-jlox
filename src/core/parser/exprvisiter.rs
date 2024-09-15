use std::rc::Rc;

use super::exprs::{Binary, ExprT, Group, Literal, Unary};

pub trait ExprVisitor<R> {
    fn visit_binary_expr(&self, expr: &Binary) -> R;
    fn visit_grouping_expr(&self, expr: &Group) -> R;
    fn visit_literal_expr(&self, expr: &Literal) -> R;
    fn visit_unary_expr(&self, expr: &Unary) -> R;
}

pub struct AstPrinter;

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        return self.parenthesize(
            &expr.operator.lexeme,
            vec![expr.left.clone(), expr.right.clone()],
        );
    }
    fn visit_grouping_expr(&self, expr: &Group) -> String {
        return self.parenthesize("group", vec![expr.expression.clone()]);
    }
    fn visit_literal_expr(&self, expr: &Literal) -> String {
        String::from(expr.value.clone())
    }
    fn visit_unary_expr(&self, expr: &Unary) -> String {
        return self.parenthesize(&expr.operator.lexeme, vec![expr.right.clone()]);
    }
}

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }
    pub fn print(&self, expr: Rc<dyn ExprT>) -> String {
        return expr.accept(self);
    }
    fn parenthesize(&self, name: &str, exprs: Vec<Rc<dyn ExprT>>) -> String {
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
}

#[cfg(test)]
mod tests_4_ast_printer {
    use super::{Binary, Group, Literal, Unary};
    use crate::core::parser::exprvisiter::AstPrinter;
    use crate::core::scanner::{LiteralType, Token, TokenType};

    #[test]
    fn test1() {
        let minus = Token {
            _type: TokenType::MINUS,
            lexeme: "-".to_owned(),
            literal: LiteralType::Nil,
            line: 1,
            offset: 1,
        };
        let multi = Token {
            _type: TokenType::STAR,
            lexeme: "*".to_owned(),
            literal: LiteralType::Nil,
            line: 1,
            offset: 1,
        };
        let expression = Binary::build(
            Unary::build(minus, Literal::build(LiteralType::Number(123.0))),
            multi,
            Group::build(Literal::build(LiteralType::Number(45.67))),
        );

        let ast_parser = AstPrinter::new();
        let output = ast_parser.print(expression);

        assert_eq!("(* (- 123) (group 45.67))", output.as_str());
    }
}
