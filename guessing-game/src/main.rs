use rand::prelude::*;
use std::io;

fn main() {
    let random_number = generate_random_number();
    println!("Please guess the number between 1 and 100!");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!("read number {}", input);
        let guessed_number = input.trim().parse::<u8>().unwrap();
        if guessed_number == random_number {
            println!("You guessed right!");
            break;
        } else if guessed_number > random_number {
            println!("You guessed higher! Guess again!");
        } else {
            println!("You guessed lower! Guess again!");
        }
    }
}

fn generate_random_number() -> u8 {
    let mut rng = thread_rng();
    rng.gen_range(1..101)
}

#[test]
fn generate_random_number_test() {
    for _ in 0..100 {
        let random_number = generate_random_number();
        assert!(random_number > 0 && random_number < 101, "Randome number should be between 1 and 100");
    }
}

