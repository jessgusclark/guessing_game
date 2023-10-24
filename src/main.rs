// Starndard(std) library io
use std::io;
use std::cmp::Ordering;
use rand::Rng;

mod colors;
use colors::{Color, format_color};

mod text_utils;
use text_utils::text_utils::{hello_mod, create_header};


const MAX_MUMBER: u32 = 100;
fn main() {
    create_header("GUESS THE NUMBER", Color::Red);

    println!("Please guess a number between 1 and {}", MAX_MUMBER);
    hello_mod();

    let secret_number = rand::thread_rng()
        .gen_range(1..=MAX_MUMBER);

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
                if num > MAX_MUMBER {
                    println!("{}", format_color("The number is between 1 and 100.", &Color::Red));
                }
                num
            },
            Err(_) => {
                println!("{}", format_color("Numbers only Please!", &Color::Yellow));
                continue
            },
        };

        // increment the guess count:
        number_of_guesses = increment(number_of_guesses);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("GUESS {}!", format_color("HIGHER", &Color::Red)),
            Ordering::Greater => println!("GUESS {}!", format_color("LOWER", &Color::Green)),
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
    println!("{} with {guesses} guesses.", format_color("YOU WIN", &Color::Green));
}

#[test]
fn test_increment() {
    assert_eq!(increment(5), 6);
    assert_eq!(increment(0), 1);
}
