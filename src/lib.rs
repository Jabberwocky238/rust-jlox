pub mod core;

use core::{interpreter::Interpreter, parser::{astprinter::AstPrinter, Parser}, scanner::Scanner};

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
    pub fn run_file(&mut self, path: &String) {
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
        println!("Running: {}", source);
        let mut scanner = Scanner::build(source);
        let tokens = scanner.scan_tokens().clone();
        // For now, just print the tokens.
        for token in tokens.iter() {
            println!("{:?}", token);
        }

        let parser: Parser = Parser::new(tokens);
        let expression = match parser.parse() {
            Ok(expr) => expr,
            Err(e) => {
                self.had_error = true;
                // Stop if there was a syntax error.
                panic!("{}", e.0);
            },
        };
        let ast_printer = AstPrinter::new();
        println!("{}", ast_printer.print(&expression));
        
        match self.interpreter.interpret(&expression) {
            Ok(output) => {
                println!("{}", output);
            },
            Err(e) => {
                self.had_error = true;
                panic!("{}", e.0);
            },
        };
    }
}

