use std::cmp::Ordering;
use std::io;

use rand::Rng;
use rand::thread_rng;

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let guess = get_guess_number();
    let guess_result = guess.cmp(&secret_number);

    match guess_result {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("You guessed: {guess}");
}

fn get_guess_number() -> u32 {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    return guess;
}
