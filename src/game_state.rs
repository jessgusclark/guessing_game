pub mod game_state {
    use rand::Rng;
    use std::cmp::Ordering;

    pub struct GameState {
        secret_number: u32,
        pub max_number: u32,
        pub lowest_guess: u32,
        pub highest_guess: u32,
        pub number_of_guesses: u32,
        pub number_of_hints: u32,
    }

    pub enum GuessOutcome {
        Equal,
        Higher,
        Lower,
        OutOfRange,
    }

    impl GameState {
        pub fn new(max_number: u32) -> Self {
            println!("Using the GameState state!");
            GameState {
                secret_number: rand::thread_rng().gen_range(1..=max_number),
                max_number,
                lowest_guess: 0,
                highest_guess: max_number,
                number_of_guesses: 0,
                number_of_hints: 0,
            }
        }

        pub fn handle_guess(&mut self, guess: u32) -> GuessOutcome {
            self.number_of_guesses += 1;

            if guess > self.max_number { return GuessOutcome::OutOfRange }

            match guess.cmp(&self.secret_number) {
                Ordering::Less => {
                    if guess > self.lowest_guess {
                        self.lowest_guess = guess;
                    }
                    GuessOutcome::Higher
                },
                Ordering::Greater => {
                    if guess < self.highest_guess {
                        self.highest_guess = guess;
                    }
                    GuessOutcome::Lower
                },
                Ordering::Equal => GuessOutcome::Equal
            }
        }

        pub fn get_a_hint(&mut self) -> [u32; 2] {
            self.number_of_hints += 1;
            [self.lowest_guess, self.highest_guess]
        }
    }
}
