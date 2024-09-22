pub mod core;

use core::interpreter::Interpreter;
use core::interpreter::AstPrinter;
use core::parser::Parser;
use core::scanner::Scanner;

pub struct Lox {
    interpreter: Interpreter,
    pub had_runtime_error: bool,
    pub had_error: bool,
}

#[derive(PartialEq, Eq)]
pub enum LoxOption {
    TOKEN,
    AST,
    INTERPRET,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            had_runtime_error: false,
            had_error: false,
        }
    }
    pub fn run_file(&mut self, path: &String, option: LoxOption) {
        // 读文件
        let source = std::fs::read_to_string(path).unwrap();
        // 调用run
        self.run(&source, option);
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
    fn run(&mut self, source: &String, option: LoxOption) {
        // println!("Running: {}", source);
        let mut scanner = Scanner::build(source);
        let tokens = scanner.scan_tokens().clone();
        if option == LoxOption::TOKEN {
            // For now, just print the tokens.
            for token in tokens.iter() {
                println!("{:?}", token);
            }
            return;
        }

        let parser: Parser = Parser::new(tokens);
        let stmts = parser.parse();

        if option == LoxOption::AST {
            let ast_printer = AstPrinter::new();
            for stmt in stmts.iter() {
                println!("[AstPrinter]: {}", ast_printer.print_stmt(&stmt));
            }
        }

        if option == LoxOption::INTERPRET {
            match self.interpreter.interpret(&stmts) {
                Ok(_) => {
                    println!("[Interpreter] end");
                },
                Err(e) => {
                    self.had_error = true;
                    panic!("{}", e.0);
                },
            };
        }
    }
}

