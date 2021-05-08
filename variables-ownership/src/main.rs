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

fn string_literals_vs_string() -> String {
    let literal = "Hello";
    let mut mut_literal = " World!";

    let mut str_hello = String::from(literal);
    str_hello.push_str(mut_literal);
    str_hello
    
    // println!("{}", mut_literal);
}

fn trim_spaces(source: &str) -> &str {
    let mut start_index = 0;
    let mut end_index = source.len();
    for c in source.chars() {
        if c == ' ' {
            start_index += 1;
        } else {
            break;
        }
    }

    for c in source.chars().rev() {
        if c == ' ' {
            end_index -= 1;
        } else {
            break;
        }
    }

    &source[start_index..end_index]
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
    assert_eq!(string_literals_vs_string(), "Hello World!");
}

#[test]
fn trim_spaces_test() {
    let test1 = "We need more Space.";
    assert_eq!(trim_spaces(test1), "We need more Space.");

    let test1 = "     We need more Space.";
    assert_eq!(trim_spaces(test1), "We need more Space.");

    let test1 = "We need more Space.      ";
    assert_eq!(trim_spaces(test1), "We need more Space.");

    let test1 = "        We need more Space.      ";
    assert_eq!(trim_spaces(test1), "We need more Space.");

    let test1 = " 🚀 is great ";
    assert_eq!(trim_spaces(test1), "🚀 is great");
}