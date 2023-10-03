// Starndard(std) library io
use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("{}", print_with_color("WELCOME!!!!", 91));
    println!("Please guess a number between 1 and 100");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    let mut number_of_guesses = 0;

    // println!("Hint, the Number is {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // check if the input is a number 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num > 100 {
                    println!("The number is between 1 and 100.");
                }
                num
            },
            Err(_) => {
                // println!("\x1b[93mNumbers only Please\x1b[0m");
                print_yellow("Numbers only Please");
                continue
            },
        };

        // increment the guess count:
        number_of_guesses = increment(number_of_guesses);

        // println!("You guessed: {guess} answer was {secret_number}");

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

fn increment(amount: u32) -> u32 {
    // will return since there is no ; at the end:
    amount + 1
}

fn win(guesses: u32) {
    println!("YOU WIN with {guesses} guesses.");
}

fn print_yellow(text: &str) {
    println!("\x1b[93m{text}\x1b[0m");
}

fn print_with_color(text: &str, color: u32) -> String {
    format!("\x1b[{color}m{text}\x1b[0m") //.to_string()
    //println!("\x1b[{color}m{text}\x1b[0m");
}

#[test]
fn test_increment() {
    assert_eq!(increment(5), 6);
    assert_eq!(increment(0), 1);
}
