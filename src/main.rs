// Starndard(std) library io
use std::io;
use std::cmp::Ordering;
use rand::Rng;

mod game_state; // Import the game_state module
use game_state::game_state::GameState; // Import the GameState struct

mod colors;
use colors::{Color, format_color};

mod text_utils;
use text_utils::utils::create_header;


const MAX_NUMBER: u32 = 100;
fn main() {
    create_header("GUESS THE NUMBER", Color::Red);

    println!("Please guess a number between 1 and {}", MAX_NUMBER);

    let secret_number = rand::thread_rng()
        .gen_range(1..=MAX_NUMBER);
    
    let mut game = GameState::new(secret_number); 

    let mut number_of_guesses = 0;

    let mut number_of_hints = 0;
    let mut highest_guess: u32 = MAX_NUMBER;
    let mut lowest_guess: u32 = 0;

    println!("Hint, the Number is {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // check if the input is a number 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num > MAX_NUMBER {
                    println!(
                        "{}", 
                        format_color(
                            &return_range(1, MAX_NUMBER),
                            &Color::Red
                        )
                    );
                }
                num
            },
            Err(_) => {
                if guess.trim() == "hint" {
                    println!("{}", return_range(lowest_guess, highest_guess));
                    number_of_hints = increment(number_of_hints);
                    continue
                }
            
                println!("{}", format_color("Numbers only Please!", &Color::Yellow));
                continue
            },
        };

        if game.handle_guess(guess) {
            win(game.number_of_guesses, game.number_of_hints);
            break;
        } else {
            println!("NOPE");
        }
    }
}

fn return_range(lowest: u32, highest: u32) -> String {
    format!("It is between {} and {}", lowest, highest)
}

fn increment(amount: u32) -> u32 {
    // will return since there is no ; at the end:
    amount + 1
}

fn win(guesses: u32, hints: u32) {
    println!("{} with {guesses} guesses and {hints} hints!", format_color("YOU WIN", &Color::Green));
}

#[test]
fn test_increment() {
    assert_eq!(increment(5), 6);
    assert_eq!(increment(0), 1);
}
