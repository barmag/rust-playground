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

#[test]
fn shadow_variable_test(){
    assert_eq!(shadow_variable(), 10);
}
