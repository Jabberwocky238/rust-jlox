mvn archetype:generate -DgroupId=com.jlox -DartifactId=jlox -DarchetypeArtifactId=maven-archetype-quickstart -DinteractiveMode=false

mvn compile

mvn exec:java -Dexec.mainClass="com.jlox.Lox"

java src\main\java\com\tool\GenerateAst.java src\main\java\com\utils

```cpp

// program        → declaration* EOF ;

// declaration    → varDecl | statement ;
// statement      → exprStmt | printStmt ;
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;

// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
// primary        → "true" | "false" | "nil" | NUMBER | STRING | "(" expression ")" | IDENTIFIER ;

// expression     → literal | unary | binary | grouping ;
// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;




// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;
```