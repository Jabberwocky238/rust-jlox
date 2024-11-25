use std::env::args;

use jlox_rust::Lox;

fn main() {
    let cmd_args: Vec<String> = args().collect();
    let mut lox = Lox::new();
    if cmd_args.len() > 2 {
        println!("Usage: jlox [script]");
        // 64 is the exit code for invalid arguments
        // for arg in cmd_args.iter() {
        //     println!("{:?}", arg);
        // }
        std::process::exit(64);
    } else if cmd_args.len() == 2 {
        lox.run_file(&cmd_args[1]);
    } else {
        lox.run_prompt();
    }
}
