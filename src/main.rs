// this project will be used for me to familiarize myself with some more rust libraries (e.g. serde_json, colorize)
mod rps;
use crate::rps::RPS;

use std::io;
use std::io::Write;

fn main() {
    let mut completed: bool = false;
    let mut iter: u8 = 0;

    let mut rps = RPS::new(true);


    while !completed {
        // clear console
        print!("\x1B[2J\x1B[1;1H");

        println!("~ Welcome to rock paper scissors ~\nYou will be facing against a simple algorithm who predicts your moves based on your history of moves");

        rps.print_statistics();

        
        // i need to read back up on how immutable/mutable references work, this code errors out if I input res.0 and res.1 into rps.update_statistics
        let mut res = rps.play_game();

        rps.update_statistics(res.0, "rock"); // currently hardcoded as rock to avoid errors
        
        

        

        print!("Would you like to play again? | Y / N |: ");
        io::stdout().flush().expect("Unable to flush stdout");

        // stores input once assigned in input_size
        let mut user_input: String = String::new();
        let input_size: usize = std::io::stdin() // read input
            .read_line(&mut user_input)
            .expect("Failed to read line");
        // remove whitespaces and other unnecessary stuff, please let me know if there is a better way to do this
        user_input = user_input.replace(" ", "",).replace("\n", "").replace("\r", "").to_uppercase();
        match user_input.as_str() {
            "Y" => {  }
            "N" => {
                println!("Exiting...");
                completed = true;
            }
            _ => {
                if iter == 0 {
                    iter += 1
                }
                else {
                    println!("Invalid input, please try again!");
                }
            }
        }
       
        // clear console
        print!("\x1B[2J\x1B[1;1H");
    }

    println!("Goodbye! :)");

    return;

}
