// Starndard(std) library io
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome!");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // check if the input is a number 
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // @todo fix this (not part of the tutorial)
    if guess > 100 {
        println!("Guess should be less than 100!");
    }

    println!("You guessed: {guess} answer was {secret_number}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You Win!"),
    }
}
