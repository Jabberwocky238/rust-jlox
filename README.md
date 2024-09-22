# jlox implementation in rust

still in progress

now at chapter 8.1: Statements and State (Statements)

[link](https://craftinginterpreters.com/statements-and-state.html#statements)

# With all my respect
[the book](https://craftinginterpreters.com/)

# unit tests
```bash
cargo test -- --test-threads=1
```

## for modules
```bash
cargo test tests_4_interpreter::
cargo test tests_4_parser::
cargo test tests_4_ast_printer::
```

## run samples
```bash
cargo run -- scripts\right\chapter7\math1.lox
cargo run -- scripts\right\chapter8\statement.lox
cargo run -- scripts\right\chapter8\globalvars.lox
cargo run -- scripts\right\chapter8\environment.lox
cargo run -- scripts\right\chapter8\assignment.lox
cargo run -- scripts\right\chapter8\scope_nest.lox
cargo run -- scripts\right\chapter8\scope_shadow.lox
```
