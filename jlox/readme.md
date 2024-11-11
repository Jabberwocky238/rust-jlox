mvn archetype:generate -DgroupId=com.jlox -DartifactId=jlox -DarchetypeArtifactId=maven-archetype-quickstart -DinteractiveMode=false

mvn compile

mvn exec:java -Dexec.mainClass="com.jlox.Lox"

java src\main\java\com\tool\GenerateAst.java src\main\java\com\utils

```bash
after chapter 8
mvn exec:java -Dexec.mainClass="com.jlox.Lox" -Dexec.args="../scripts/scope.lox"
after chapter 9
mvn exec:java -Dexec.mainClass="com.jlox.Lox" -Dexec.args="../scripts/for.lox"
after chapter 10
mvn exec:java -Dexec.mainClass="com.jlox.Lox" -Dexec.args="../scripts/func.lox"
mvn exec:java -Dexec.mainClass="com.jlox.Lox" -Dexec.args="../scripts/while_in_func.lox"
mvn exec:java -Dexec.mainClass="com.jlox.Lox" -Dexec.args="../scripts/closure.lox"
```


```cpp

// program        → declaration* EOF ;

// declaration    → funDecl | varDecl | statement ;
funDecl        → "fun" function ;
function       → IDENTIFIER "(" parameters? ")" block ;
parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
// statement      → exprStmt | printStmt | ifStmt | whileStmt | returnStmt | block ;


// whileStmt      → "while" "(" expression ")" statement ;
// ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;
returnStmt     → "return" expression? ";" ;
// block          → "{" declaration* "}" ;

// expression     → assignment ;
// assignment     → IDENTIFIER "=" assignment | logic_or ;
// logic_or       → logic_and ( "or" logic_and )* ;
// logic_and      → equality ( "and" equality )* ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | call ;
call           → primary ( "(" arguments? ")" )* ;
arguments      → expression ( "," expression )* ;

// primary        → literal | "(" expression ")" | IDENTIFIER ;
// grouping       → "(" expression ")" ;
// binary         → expression operator expression ;
// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;


```