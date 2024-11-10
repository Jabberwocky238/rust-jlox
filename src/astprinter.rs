use std::rc::Rc;

use crate::gen::*;

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
}

impl_stmt_visitable! {
    <String>, 
    (Expression, expression),
    (Print, print),
    (Var, var),
    (Block, block),
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
        expr.to_string()
    }
    fn visit_unary(&self, expr: &Unary) -> String {
        return self.parenthesize(&expr.operator.lexeme, &[&expr.right]);
    }
    
    fn visit_variable(&self, expr: &Variable) -> String {
        String::from(expr.name.lexeme.clone())
    }
    
    fn visit_assign(&self, stmt: &Assign) -> String {
        String::from(format!("{} = {}", stmt.name.lexeme, stmt.value.accept(self)))
    }
}

impl StmtVisitor<String> for AstPrinter {
    fn visit_expression(&self, stmt: &Expression) -> String {
        return format!("( {} )", stmt.expression.accept(self));
    }
    fn visit_print(&self, stmt: &Print) -> String {
        return self.parenthesize(&"print", &[&stmt.expression]);
    }
    fn visit_var(&self, stmt: &Var) -> String {
        match &stmt.initializer {
            Some(expr) => {
                return format!("( var {} = {} )", stmt.name.lexeme, expr.accept(self));
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
            let stmt = stmt.accept(self);
            string_builder.push(stmt);
        });
        string_builder.push("\n}".to_owned());
        return string_builder.join("");
    }
}

impl AstPrinter 
where Self: ExprVisitor<String> 
{
    pub fn new() -> Self {
        AstPrinter {}
    }
    pub fn print_expr(&self, expr: &Rc<Expr>) -> String {
        return expr.accept(self);
    }
    pub fn print_stmt(&self, stmt: &Rc<Stmt>) -> String {
        return stmt.accept(self);
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
    use crate::gen::*;
    use crate::astprinter::AstPrinter;
    use crate::gen::Expr;
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
        let output = ast_parser.print_expr(&expression);

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
