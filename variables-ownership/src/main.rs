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

#[test]
fn shadow_variable_test(){
    assert_eq!(shadow_variable(), 10);
}

#[test]
fn shadow_with_type_change_test(){
    assert_eq!(shadow_with_changing_type(), 42.5);
}
