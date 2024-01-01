
mod constants;
mod lib;

use crate::constants::QUIT;
use crate::lib::TAF;


// don't be tempted to use tokio, we only need one request/thread, so it's blocking
// https://tokio.rs/tokio/tutorial
fn main() {
    let mut user_input = String::new();

    while user_input.trim() != QUIT {
        println!("Enter an airport code: ");
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        if user_input.trim() == QUIT {
            println!("Goodbye!");
            break;
        }
        if user_input.trim().len() < 3 || user_input.trim().len() > 4 {
            println!("Please enter a valid airport code");
            continue;
        }
        if user_input.trim().len() == 3 {
            user_input = format!("K{}", user_input);
        }
        println!("Getting Terminal Aerodrome Forecast: {}", user_input);

        let taf_str: String = TAF::fetch_taf(user_input.trim());
        let taf = TAF::parse_taf(taf_str);
        println!("{}", taf.to_string());
        user_input = String::new();
    }
}
