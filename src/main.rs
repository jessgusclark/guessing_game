// Starndard(std) library io
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("{}", format_color("======== WELCOME ========", Color::Red));
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
                    println!("{}", format_color("The number is between 1 and 100.", Color::Red));
                }
                num
            },
            Err(_) => {
                println!("{}", format_color("Numbers only Please!", Color::Yellow));
                continue
            },
        };

        // increment the guess count:
        number_of_guesses = increment(number_of_guesses);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("GUESS {}!", format_color("HIGHER", Color::Red)),
            Ordering::Greater => println!("GUESS {}!", format_color("LOWER", Color::Green)),
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
    println!("{} with {guesses} guesses.", format_color("YOU WIN", Color::Green));
}

enum Color {
    Red,
    Green,
    Yellow,
}

fn match_color(color: Color) -> u8 {
    match color {
        Color::Red => 91,
        Color::Green => 92,
        Color::Yellow => 93,
    }
}

fn format_color(text: &str, color: Color) -> String {
    format!("\x1b[{}m{text}\x1b[0m", match_color(color))
}
