use std::process;

mod print_help;
use print_help::print_help;

mod get_max_number;
use get_max_number::get_max_number;

pub fn handle_user_parameters(args: Vec<String>) -> u32 {
    let mut max_number: u32 = 100;
    for arg in args.iter() {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }

            "--version" | "-v" => {
                println!("Version: 0.1.0");
            }

            arg if arg.starts_with("--max_number=") =>
                max_number = get_max_number(arg, 100),

            // need to handle the default match, in this case, ignore it.
            _ => {}
        }
    }

    // return the max number:
    max_number
}
