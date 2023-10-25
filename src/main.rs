// Starndard(std) library io
use std::io;

mod game_state;
use game_state::game_state::{GameState, GuessOutcome};

mod colors;
use colors::{Color, format_color};

mod text_utils;
use text_utils::utils::create_header;

const MAX_NUMBER: u32 = 100;
fn main() {
    create_header("GUESS THE NUMBER", Color::Red);

    println!("Please guess a number between 1 and {}", MAX_NUMBER);
    
    let mut game = GameState::new(MAX_NUMBER); 

    // println!("Hint, the Number is {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // check if the input is a number 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim() == "hint" {
                    return_range(game.get_a_hint());
                    continue
                }
            
                println!("{}", format_color("Numbers only Please!", &Color::Yellow));
                continue
            },
        };

        match game.handle_guess(guess) {
            GuessOutcome::Lower =>
                println!("{}", format_color(&"Guess Lower", &Color::Red)),
            GuessOutcome::Higher =>
                println!("{}", format_color(&"Guess Higher", &Color::Green)),
            GuessOutcome::OutOfRange => {
                println!(
                    "Guess out of range. {}", 
                    format_color(
                        &return_range([1, MAX_NUMBER]),
                        &Color::Red
                    )
                );
            },
            GuessOutcome::Equal => {
                win(&game.number_of_guesses, &game.number_of_hints);
                break;
            }
        }
    }
}

fn return_range(range: [u32; 2]) -> String {
    format!("It is between {} and {}", range[0], range[1])
}

fn win(guesses: &u32, hints: &u32) {
    println!("{} with {guesses} guesses and {hints} hints!", format_color("YOU WIN", &Color::Green));
}
