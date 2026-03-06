use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input you guess.");

    let secret_number: u32 = rand::random_range(0..=100);
    println!("The secret number is: {secret_number}");

    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input!");
    let guess_number: u32 = guess.trim().parse().expect("Failed to parse user input!");

    match guess_number.cmp(&secret_number) {
        cmp::Ordering::Less => println!("Too small!"),
        cmp::Ordering::Greater => println!("Too big!"),
        cmp::Ordering::Equal => println!("You win!"),
    }

    println!("You guessed: {guess}")
}
