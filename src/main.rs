// Starndard(std) library io
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    const MAX_MUMBER: u32 = 100;

    // println!("{}", format_color("======== WELCOME ========", Color::Red));
    create_header("WELCOME", Color::Yellow);

    println!("Please guess a number between 1 and {}", MAX_MUMBER);

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

fn create_header(text: &str, color: Color) {
    let local_color = color;
    let text_length: usize = text.len() + 9;
    let line = format_color(&write_line(text_length), local_color);

    println!("{}", line);
    // println!("||  {text}  ||");
    println!("||  {}  ||", format_color(text, Color::Red));
    println!("{}", line);
}

fn write_line(length: usize) -> String {
    let mut return_value:String = String::new();

    for _i in 1..length {
        // return_value = return_value + "=";
        return_value.push_str("=");
    }
    return return_value;
}

#[test]
fn test_increment() {
    assert_eq!(increment(5), 6);
    assert_eq!(increment(0), 1);
}

#[test]
fn test_format_color() {
    assert_eq!(format_color("hello", Color::Red), "\x1b[91mhello\x1b[0m");
    assert_eq!(format_color("hello", Color::Green), "\x1b[92mhello\x1b[0m");
    assert_eq!(format_color("hello", Color::Yellow), "\x1b[93mhello\x1b[0m");
}
