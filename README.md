# jlox implementation in rust

still in progress

ended at chapter 7: Evaluating Expressions

now at chapter 8: Statements and State

# unit tests
cargo test -- --test-threads=1

## for modules
cargo test tests_4_interpreter::

cargo test tests_4_parser::

cargo test tests_4_ast_printer::

## samples
cargo run -- scripts\right\chapter7\math1.lox
cargo run -- scripts\right\chapter8\statement.lox
cargo run -- scripts\right\chapter8\globalvars.lox
cargo run -- scripts\right\chapter8\environment.lox
cargo run -- scripts\right\chapter8\assignment.lox
cargo run -- scripts\right\chapter8\scope_nest.lox
cargo run -- scripts\right\chapter8\scope_shadow.lox

# run
cargo run -- math1.lox

