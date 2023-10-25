pub mod game_logic {
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
        pub fn new(secret_number: u32, max_number: u32) -> Self {
            GameState {
                secret_number,
                max_number,
                lowest_guess: 0,
                highest_guess: max_number,
                number_of_guesses: 0,
                number_of_hints: 0,
            }
        }

        pub fn handle_guess(&mut self, guess: u32) -> GuessOutcome {
            if guess > self.max_number { return GuessOutcome::OutOfRange }

            // number of valid guesses:
            self.number_of_guesses += 1;

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

#[cfg(test)]
mod tests {
    use crate::{GameState, game_state::game_logic::GuessOutcome};

    #[test]
    fn test_new_game_state() {
        // Test the new constructor
        let game = GameState::new(50, 100);
        assert_eq!(game.max_number, 100);
        assert_eq!(game.highest_guess, 100);
        assert_eq!(game.lowest_guess, 0);
    }

    #[test]
    fn test_handle_guess() {
        let mut game = GameState::new(50, 100);
        matches!(game.handle_guess(75), GuessOutcome::Lower);
        matches!(game.handle_guess(25), GuessOutcome::Higher);

        assert_eq!(game.number_of_guesses, 2);

        matches!(game.handle_guess(150), GuessOutcome::OutOfRange);
        matches!(game.handle_guess(50), GuessOutcome::Equal);

        // 3 valid guesses since 150 is out of range:
        assert_eq!(game.number_of_guesses, 3);
    }

    #[test]
    fn test_get_a_hint() {
        let mut game = GameState::new(50, 100);
        game.handle_guess(75);
        game.handle_guess(25);

        let hint = game.get_a_hint();
        assert_eq!(hint, [25,75]);
        assert_eq!(game.number_of_hints, 1);
    }
}
