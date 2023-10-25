pub mod game_state {
    use std::cmp::Ordering;

    pub struct GameState {
        secret_number: u32,
        pub lowest_guess: u32,
        pub highest_guess: u32,
        pub number_of_guesses: u32,
        pub number_of_hints: u32,
    }

    pub enum GuessOutcome {
        Equal,
        Higher,
        Lower,
    }

    impl GameState {
        pub fn new(secret_number: u32) -> Self {
            println!("Using the GameState state!");   
            GameState {
                secret_number,
                lowest_guess: 0,
                highest_guess: 100,
                number_of_guesses: 0,
                number_of_hints: 0,
            }
        }

        pub fn handle_guess(&mut self, guess: u32) -> &'static str {
            self.number_of_guesses += self.number_of_guesses;

            match guess.cmp(&self.secret_number) {
                Ordering::Less => {
                    // println!("GUESS {}!", format_color("HIGHER", &Color::Red));
                    if guess > self.lowest_guess {
                        self.lowest_guess = guess;
                    }
                    // GuessOutcome::Higher
                    "Guess HIGHER"
                },
                Ordering::Greater => {
                    // println!("GUESS {}!", format_color("LOWER", &Color::Green));
                    if guess < self.highest_guess {
                        self.highest_guess = guess;
                    }
                    "Guess LOWER"
                },
                Ordering::Equal => "Equal"
            }
        }
    }
}
