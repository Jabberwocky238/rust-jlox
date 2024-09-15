pub mod core;

use core::{parser::{exprvisiter::AstPrinter, Parser}, scanner::Scanner};

pub struct Lox;

impl Lox {
    pub fn run_file(path: &String) {
        // 读文件
        let source = std::fs::read_to_string(path).unwrap();
        // 调用run
        Self::run(&source);
    }
    pub fn run_prompt() {
        // InputStreamReader input = new InputStreamReader(System.in);
        // BufferedReader reader = new BufferedReader(input);

        // for (;;) { 
        //     System.out.print("> ");
        //     String line = reader.readLine();
        //     if (line == null) break;
        //     run(line);
        // }
    }
    fn run(source: &String) {
        let mut scanner = Scanner::build(source);
        let tokens = scanner.scan_tokens().clone();
        // For now, just print the tokens.
        for token in tokens.iter() {
            println!("{:?}", token);
        }

        let parser: Parser = Parser::new(tokens);
        let ast_printer = AstPrinter::new();
        let expression = parser.parse().unwrap();
        ast_printer.print(expression);
    }
}

