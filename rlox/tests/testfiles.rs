use jlox_rust::Lox;

#[test]
fn test_print() {
    let mut lox = Lox::new();
    lox.run_file("./samples/print.lox");
}

#[test]
fn test_for_if() {
    let mut lox = Lox::new();
    lox.run_file("./samples/for_if.lox");
}

#[test]
fn test_scope() {
    let mut lox = Lox::new();
    lox.run_file("./samples/scope.lox");
}

#[test]
fn test_func() {
    let mut lox = Lox::new();
    lox.run_file("./samples/func.lox");
}

#[test]
fn test_closure() {
    let mut lox = Lox::new();
    lox.run_file("./samples/closure.lox");
}

#[test]
fn test_binding() {
    let mut lox = Lox::new();
    lox.run_file("./samples/binding.lox");
}