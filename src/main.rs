// Starndard(std) library io
use std::io;
use rand::Rng;

mod game_state;
use game_state::game_state::{GameState};

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
                    let hint = game.get_a_hint();
                    return_range(hint[0], hint[1]);
                    continue
                }
            
                println!("{}", format_color("Numbers only Please!", &Color::Yellow));
                continue
            },
        };

        let outcome = game.handle_guess(guess);
        if outcome == "Equal" {
            win(&game.number_of_guesses, &game.number_of_hints);
            break;
        } else {
            println!("{outcome}")
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

fn win(guesses: &u32, hints: &u32) {
    println!("{} with {guesses} guesses and {hints} hints!", format_color("YOU WIN", &Color::Green));
}

#[test]
fn test_increment() {
    assert_eq!(increment(5), 6);
    assert_eq!(increment(0), 1);
}
