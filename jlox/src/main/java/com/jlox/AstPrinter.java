package com.jlox;

import com.utils.Expr;
import com.utils.Expr.Assign;
import com.utils.Expr.Binary;
import com.utils.Expr.Call;
import com.utils.Expr.Grouping;
import com.utils.Expr.Literal;
import com.utils.Expr.Logical;
import com.utils.Expr.Unary;
import com.utils.Expr.Variable;

class AstPrinter implements Expr.Visitor<String> {

  private String parenthesize(String name, Expr... exprs) {
    StringBuilder builder = new StringBuilder();

    builder.append("(").append(name);
    for (Expr expr : exprs) {
      builder.append(" ");
      builder.append(expr.accept(this));
    }
    builder.append(")");

    return builder.toString();
  }

  public static void main(String[] args) {
    Expr expression = new Expr.Binary(
        new Expr.Unary(
            new Token(TokenType.MINUS, "-", null, 1),
            new Expr.Literal(123)),
        new Token(TokenType.STAR, "*", null, 1),
        new Expr.Grouping(
            new Expr.Literal(45.67)));

    System.out.println(new AstPrinter().print(expression));
  }

  String print(Expr expr) {
    return expr.accept(this);
  }

  @Override
  public String visitBinaryExpr(Binary expr) {
    return parenthesize(expr.operator.lexeme,
        expr.left, expr.right);
  }

  @Override
  public String visitGroupingExpr(Grouping expr) {
    return parenthesize("group", expr.expression);
  }

  @Override
  public String visitLiteralExpr(Literal expr) {
    if (expr.value == null)
      return "nil";
    return expr.value.toString();
  }

  @Override
  public String visitUnaryExpr(Unary expr) {
    return parenthesize(expr.operator.lexeme, expr.right);
  }

  @Override
  public String visitAssignExpr(Assign expr) {
    // TODO Auto-generated method stub
    throw new UnsupportedOperationException("Unimplemented method 'visitAssignExpr'");
  }

  @Override
  public String visitVariableExpr(Variable expr) {
    // TODO Auto-generated method stub
    throw new UnsupportedOperationException("Unimplemented method 'visitVariableExpr'");
  }

  @Override
  public String visitLogicalExpr(Logical expr) {
    // TODO Auto-generated method stub
    throw new UnsupportedOperationException("Unimplemented method 'visitLogicalExpr'");
  }

  @Override
  public String visitCallExpr(Call expr) {
    // TODO Auto-generated method stub
    throw new UnsupportedOperationException("Unimplemented method 'visitCallExpr'");
  }
}