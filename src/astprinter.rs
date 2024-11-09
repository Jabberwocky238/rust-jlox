use std::rc::Rc;

use crate::exprs::Expr;
use crate::exprs::Visitable;
use crate::exprs::Visitor;
use crate::exprs::Binary;
use crate::exprs::Group;
use crate::exprs::Literal;
use crate::exprs::Unary;
use crate::impl_visitable;

pub struct AstPrinter;

impl_visitable! {
    impl <String> for Expr, 
    (Binary, binary),
    (Group, grouping),
    (Literal, literal),
    (Unary, unary),
}

// impl Visitable<String> for Expr {
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

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        return self.parenthesize(
            &expr.operator.lexeme,
            &[&expr.left, &expr.right],
        );
    }
    fn visit_grouping_expr(&self, expr: &Group) -> String {
        return self.parenthesize("group", &[&expr.expression]);
    }
    fn visit_literal_expr(&self, expr: &Literal) -> String {
        String::from(expr.value.clone())
    }
    fn visit_unary_expr(&self, expr: &Unary) -> String {
        return self.parenthesize(&expr.operator.lexeme, &[&expr.right]);
    }
}

impl AstPrinter 
where Self: Visitor<String> 
{
    pub fn new() -> Self {
        AstPrinter {}
    }
    pub fn print(&self, expr: &Rc<Expr>) -> String {
        return expr.accept(self);
    }
    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
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
    use std::rc::Rc;
    use crate::exprs::*;
    use crate::astprinter::AstPrinter;
    use crate::exprs::Expr;
    use crate::scanner::{LiteralType, Token, TokenType};

    fn easy_number(num: f64) -> Rc<Expr> {
        Literal::build(LiteralType::Number(num))
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
        
        let expression = Binary::build(
            Unary::build(minus, _123),
            multi,
            Group::build(_45dot67),
        );

        let ast_parser = AstPrinter::new();
        let output = ast_parser.print(&expression);

        assert_eq!("(* (- 123) (group 45.67))", output.as_str());
    }

    // #[test]
    // fn test2() {
    //     let or = easy_token(TokenType::OR, "or");
    //     let and = easy_token(TokenType::AND, "and");
    //     let _123 = easy_number(123.0);
    //     let _456 = easy_number(456.0);
    //     let _true = easy_token(TokenType::TRUE, "true");

    //     let expression = Binary::build(
    //         Unary::build(minus, Literal::build(LiteralType::Number(123.0))),
    //         multi,
    //         Group::build(Literal::build(LiteralType::Number(45.67))),
    //     );

    //     let ast_parser = AstPrinter::new();
    //     let output = ast_parser.print(&expression);

    //     assert_eq!("(* (- 123) (group 45.67))", output.as_str());
    // }
}
