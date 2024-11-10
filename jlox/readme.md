mvn archetype:generate -DgroupId=com.jlox -DartifactId=jlox -DarchetypeArtifactId=maven-archetype-quickstart -DinteractiveMode=false

mvn compile

mvn exec:java -Dexec.mainClass="com.jlox.Lox"

java src\main\java\com\tool\GenerateAst.java src\main\java\com\utils

mvn exec:java -Dexec.mainClass="com.jlox.Lox" -Dexec.args="../scripts/scope.lox"

```cpp

// program        → declaration* EOF ;

// declaration    → varDecl | statement ;
// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
// statement      → exprStmt | printStmt | ifStmt | whileStmt | block ;

// whileStmt      → "while" "(" expression ")" statement ;
// ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;
// block          → "{" declaration* "}" ;

// expression     → assignment ;
// assignment     → IDENTIFIER "=" assignment | logic_or ;
// logic_or       → logic_and ( "or" logic_and )* ;
// logic_and      → equality ( "and" equality )* ;

// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;

// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | primary ;

// primary        → "true" | "false" | "nil" | NUMBER | STRING | "(" expression ")" | IDENTIFIER ;

```