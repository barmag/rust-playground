use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

fn shadow_variable() -> i32 {
    let test = 5;
    // shadow test
    // the test variable is not mutable, but the let statement shadows the variable
    let test = 10;
    return test;
}
fn shadow_with_changing_type() -> f64 {
    let test: i32 = 42;

    let test: f64 = 42.5;
    return test;
}

fn shadow_with_scope() -> i32 {
    let test = 42;
    {
        let test = 42.5;
    }
    return test;
}

fn string_literals_vs_String() -> String {
    let literal = "Hello";
    let mut mut_literal = " World!";

    let mut str_hello = String::from(literal);
    str_hello.push_str(mut_literal);
    str_hello
    
    // println!("{}", mut_literal);
}

#[test]
fn shadow_variable_test(){
    assert_eq!(shadow_variable(), 10);
}

#[test]
fn shadow_with_type_change_test(){
    assert_eq!(shadow_with_changing_type(), 42.5);
}

#[test]
fn shadow_with_scopes_test(){
    assert_eq!(shadow_with_scope(), 42);
}

#[test]
fn string_literal_test(){
    assert_eq!(string_literals_vs_String(), "Hello World!");
}
