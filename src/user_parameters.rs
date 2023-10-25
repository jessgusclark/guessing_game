pub fn handle_user_parameters(args: Vec<String>) -> u32 {
    let mut max_number: u32 = 100;
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
                println!("  -v, --version   Gets the version number");
                println!("");
            }

            "--version" | "-v" => {
                println!("Version: 0.1.0");
            }

            arg if arg.starts_with("--max_number=") => {
                println!("handing the max number");
                // splits the contents into an Iterator with two parts
                // the nth() gets the second part (i.e. 1). Since this 
                // is not an array, you can't select it with [1]:
                if let Some(number) = arg.split('=').nth(1) {
                    println!("it is some number: {}", number);
                    if let Ok(number) = number.parse::<u32>() {
                        max_number = number;
                    } else {
                        println!("Error parsing max_number");
                    }
                }
            }            

            // need to handle the default match, in this case, ignore it.
            _ => {}
        }
    }

    // return the max number:
    max_number
}