# rule 
program        → declaration* EOF ;
block          → "{" declaration* "}" ;
declaration    → varDecl | statement ;
statement      → exprStmt | printStmt | block ;
**exprStmt**   → expression ";" ;
**printStmt**  → "print" expression ";" ;

varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
expression     → literal | unary | binary | grouping ;
**literal**    → NUMBER | STRING | "true" | "false" | "nil" ;
**grouping**   → "(" expression ")" ;
**unary**      → ( "-" | "!" ) expression ;
**binary**     → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;


# parser
expression     → assignment ;
assignment     → IDENTIFIER "=" assignment | equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;


