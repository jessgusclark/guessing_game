pub fn handle_user_parameters(args: Vec<String>) {
    for arg in args.iter() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("Jesse's Guessing Game!");
                println!("");
                println!("  Select a number between 1 and 100");
                println!("  Too see the range of numbers already guessed, type hint.");
                println!("  Try to guess the number with the least number of guesses.");
                println!("");
                println!("Options:");
                println!("  -h, --help      Prints this message.");
                println!("");
            }

            "--version" | "-v" => {
                println!("Version: 1.0");
            }

            // need to handle the default match, in this case, ignore it.
            _ => {}
        }
    }
}