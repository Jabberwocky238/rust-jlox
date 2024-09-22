use std::env::args;

use jlox_rust::{Lox, LoxOption};

fn main() {
    let cmd_args: Vec<String> = args().collect();
    let mut lox = Lox::new();
    if cmd_args.len() > 3 {
        println!("Usage: jlox [script] [OPTIONS=token|ast|interpret]");
        // 64 is the exit code for invalid arguments
        // for arg in cmd_args.iter() {
        //     println!("{:?}", arg);
        // }
        std::process::exit(64);
    } else if cmd_args.len() == 2 {
        lox.run_file(&cmd_args[1], LoxOption::INTERPRET);
    } 
    else if cmd_args.len() == 3 {
        let option = match cmd_args[2].as_str() {
            "token" => LoxOption::TOKEN,
            "ast" => LoxOption::AST,
            "interpret" => LoxOption::INTERPRET,
            _ => panic!("Invalid option: {}", cmd_args[2])
        };
        lox.run_file(&cmd_args[1], option);
    } else {
        lox.run_prompt();
    }
}
