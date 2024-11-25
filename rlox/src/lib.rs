mod errors;
mod scanner;
mod parser;
mod interpreter;
mod astprinter;
mod ast;
mod environment;
mod token;
mod function;
mod scope_resolver;

use interpreter::Interpreter;
use parser::Parser;
use astprinter::AstPrinter;
use scanner::Scanner;
use scope_resolver::ScopeResolver;

pub struct Lox {
    interpreter: Interpreter,
    pub had_runtime_error: bool,
    pub had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            had_runtime_error: false,
            had_error: false,
        }
    }
    pub fn run_file(&mut self, path: &str) {
        // 读文件
        let source = std::fs::read_to_string(path).unwrap();
        // 调用run
        self.run(&source);
    }
    pub fn run_prompt(&mut self) {
        // InputStreamReader input = new InputStreamReader(System.in);
        // BufferedReader reader = new BufferedReader(input);

        // for (;;) { 
        //     System.out.print("> ");
        //     String line = reader.readLine();
        //     if (line == null) break;
        //     run(line);
        // }
    }
    fn run(&mut self, source: &String) {
        // println!("Running: {}", source);
        
        let scanner = Scanner::build(source);
        let tokens = scanner.scan_tokens();
        // scanner dropped here

        // For now, just print the tokens.
        // for token in tokens.iter() {
        //     println!("{:?}", token);
        // }

        let parser: Parser = Parser::new(tokens);
        let stmts = match parser.parse() {
            Ok(stmts) => stmts,
            Err(e) => {
                self.had_error = true;
                // Stop if there was a syntax error.
                panic!("{}", e.0);
            },
        };
        // parser dropped here

        // let ast_printer = AstPrinter::new();
        // for stmt in stmts.iter() {
        //     println!("{}", ast_printer.print_stmt(stmt.clone()));
        // }
        let resolver = ScopeResolver::new(&mut self.interpreter);
        resolver.resolve(&stmts);
        
        match self.interpreter.interpret(&stmts) {
            Ok(_) => {},
            Err(e) => {
                self.had_error = true;
                panic!("{:?}", e);
            },
        };
    }
}

