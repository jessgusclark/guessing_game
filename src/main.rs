// Starndard(std) library io
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome!");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    let mut number_of_guesses = 0;

    println!("Hint, the Number is {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // check if the input is a number 
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        number_of_guesses+= 1;

        // @todo fix this (not part of the tutorial)
        if guess > 100 {
            println!("Guess should be less than 100!");
            break;
        }

        println!("You guessed: {guess} answer was {secret_number}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too Small!"),
            Ordering::Greater => println!("{guess} is too Big!"),
            Ordering::Equal => {
                win(number_of_guesses);
                break;
            }
        }
    }
}

fn win(guesses: u32) {
    println!("YOU WIN with {guesses} guesses.");
}
