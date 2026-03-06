use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input you guess.");

    let secret_number = rand::random_range(0..=100);
    println!("The secret number is: {secret_number}");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input!");

    println!("You guessed: {guess}")
}
