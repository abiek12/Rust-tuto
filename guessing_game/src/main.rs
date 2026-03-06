use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::random_range(0..=100);

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input!");
        let guess_number: u32 = guess.trim().parse().expect("Failed to parse user input!");
        println!("You guessed: {guess}");

        match guess_number.cmp(&secret_number) {
            cmp::Ordering::Less => println!("Too small!"),
            cmp::Ordering::Greater => println!("Too big!"),
            cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
