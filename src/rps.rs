use serde_json;
use serde::{Serialize, Deserialize};

use rand::Rng;
use std::fs;
use colored::Colorize;

mod user_information;
use user_information::UserInformation;

use self::user_information::{MoveHistories, UserWins, UserWinRate};

#[derive(Serialize, Deserialize, Clone)]
pub struct RPS {
    path: &'static str,
    is_debugging: bool,
    // all histories are percentages of what the user's past moves are
    user_information: UserInformation,
}

impl RPS {
    pub fn new (is_debugging: bool) -> Self {
        let path = "src/data.json";
        // parse data.json into a String
        let data = fs::read_to_string(path)
            .expect("Unable to read 'data.json' file. Maybe the file doesn't exist?");

        // convert parsed String into json format
        let mut json: UserInformation = serde_json::from_str(&data)
            .expect("Error parsing JSON."); // remember the question mark in future projects, need to learn what it means :

        Self {
            path: path,
            is_debugging,
            // i do not like how messy this is, if i can make this easier to read in the future I will
            // user_information.move_histories contains the amount of times a user moved
            // user
            user_information: UserInformation {
                user_win_rate: UserWinRate { overall: (json.user_win_rate.overall), rock: (json.user_win_rate.rock), paper: (json.user_win_rate.paper), scissors: (json.user_win_rate.scissors) },
                user_wins: UserWins { total: (json.user_wins.total), rock: (json.user_wins.rock), paper: (json.user_wins.paper), scissors: (json.user_wins.scissors) },
                move_histories: MoveHistories { total: (json.move_histories.total), rock: (json.move_histories.rock), paper: (json.move_histories.paper), scissors: (json.move_histories.scissors) },
            },
        }
    }

    pub fn play_game (&self) -> (u8, &str) { // 0 for failure, 1 for success
        let user_input = self.get_user_input();

        // ensure that user input isn't invalid
        if user_input == None {
            panic!("Hopefully this should never happen?")
        }

        let user_input_u: &str = user_input.unwrap(); // slightly more performant even though that doesn't matter at all in this scenario

        let winner = self.determine_winner(user_input_u);

        self.print_winner(&winner.0, &winner.1); // inputting who won and the move the ALGORITHM made

        return winner; // inputting who won and the move the USER made
    }

    // none of this is really ai, its just an easy way to understand what is happening
    fn determine_winner (&self, user_input: &str) -> (u8, &str) { // 0 = AI, 1 = human, 2 = draw, 3 invalid input
        match user_input {
            "rock" => {
                // now that we have user input, generate an AI move and update results
                let ai_move: &str = self.generate_move();
                match ai_move {
                    "rock" => return (2, ai_move), // draw
                    "paper" => return (0, ai_move), // AI wins
                    _ => return (1, ai_move) // human wins
                }
            }
            "paper" => {
                let ai_move: &str = self.generate_move();
                match ai_move {
                    "rock" => return (1, ai_move), // human wins
                    "paper" => return (2, ai_move), // draw
                    _ => return (0, ai_move) // AI wins
                }
            }
            "scissors" => {
                let ai_move: &str = self.generate_move();
                match ai_move {
                    "rock" => return (0, ai_move), // AI wins
                    "paper" => return (1, ai_move), // human wins
                    _ => return (2, ai_move) // draw
                }
            }
            _ => return (3, "invalid") // invalid input
        }
    }

    fn print_winner (&self, winner: &u8, ai_move: &str) { // winner inputs -> 0 = AI, 1 = human, 2 = draw, 3 invalid input
        match winner {
            0 => {
                println!("The AI picked {}, you lost!", ai_move)
            }
            1 => {
                println!("The AI picked {}, you won!", ai_move)
            }
            2 => {
                println!("The AI also picked {}, you tied!", ai_move)
            }
            3 => {
                panic!("Input was invalid when printing who won, this should never happen, report to repository owner.")
            }
            _ => {
                panic!("This should never happen,report to repository owner.")
            }
        }
    }

    // statistically, over a really long period of time, the algorithm will have a positive winrate because the user will tend to pick a certain thing more often than others.
    fn generate_move (&self) -> &str {
        // add logic here which generates a move based on historic match stats.
        let mut rng = rand::thread_rng();

        let move_histories_table: [f64; 4] = [
            self.get_move_history(0), // total
            self.get_move_history(1), // rock
            self.get_move_history(2), // paper
            self.get_move_history(3) // scissors
        ];

        // rock / total, paper / total, scissors / total, yields frequency
        let move_frequency_table: [f64; 3] = [
            (move_histories_table[1] / move_histories_table[0]) as f64,
            (move_histories_table[2] / move_histories_table[0]) as f64,
            (move_histories_table[3] / move_histories_table[0]) as f64
        ];

        let total: f64 = move_frequency_table.iter().sum();

        let rand_int: f64 = rng.gen_range(0.0..=total);

        if self.is_debugging { println!("Move Frequency -> {:?} ", move_frequency_table) }

        let mut index: usize = 0;
        let mut sum: f64 = 0.0;

        // you have sum, add frequency to sum, check if rand_int is less than that sum, repeat until you can find the index.
        // this should yield a statistically higher win rate for the algorithm
        for (i, freq) in move_frequency_table.iter().enumerate() {
          sum += freq;
        
          if rand_int < sum {
            index = i;
            break;
          }
        }

        match index {
            0 => return "paper",
            1 => return "scissors",
            2 => return "rock",
            _ => panic!("Index out of range in generate_move")
        }
    }

    pub fn update_statistics (&mut self, game_result: u8, user_input: &str) -> bool { // game_result -> 0 = AI, 1 = human, 2 = draw, 3 invalid input
        // i need to make this less completely and udderly awful, i'm sure theres a rust function for this that i'm forgetting
        self.user_information.move_histories.total = self.user_information.move_histories.rock + self.user_information.move_histories.paper + self.user_information.move_histories.scissors;


        // update total moves made and specific move made
        self.user_information.move_histories.total += 1.0;
        match user_input {
            "rock" => {
                self.user_information.move_histories.rock += 1.0;

                if game_result == 1 { // if user won
                    self.user_information.user_wins.total += 1.0;
                    self.user_information.user_wins.rock += 1.0;
                }
            }
            "paper" => {
                self.user_information.move_histories.paper += 1.0;

                if game_result == 1 { // if user won
                    self.user_information.user_wins.total += 1.0;
                    self.user_information.user_wins.paper += 1.0;
                }
            }
            "scissors" => {
                self.user_information.move_histories.scissors += 1.0;

                if game_result == 1 {
                    self.user_information.user_wins.total += 1.0;
                    self.user_information.user_wins.scissors += 1.0;
                }
            }
            _ => panic!("This should never happen, check update_statistics")
        }

        // overall win rate = total wins / total games played
        self.user_information.user_win_rate.overall = self.user_information.user_wins.total / self.user_information.move_histories.total;

        // convert RPS to json
        let json = serde_json::to_string_pretty(self);

        // write json to file
        //fs::write(self.path, json)
            //.expect("Failed to save data to json");
        return true;
    }

    pub fn print_statistics (&self) {
        if self.is_debugging { self.print_user_win_rate()}
        if self.is_debugging { self.print_move_history() };
    }

    fn get_user_win_rate (&self, index: usize) -> f64 {
        match index {
            0 => { return self.user_information.user_win_rate.overall },
            1 => { return self.user_information.user_win_rate.rock }, 
            2 => { return self.user_information.user_win_rate.paper },
            3 => { return self.user_information.user_win_rate.scissors }
            _ => { panic!("you input something wrong in get_win_rate stupid") } // if you are reviewing my code this makes me giggle if i mess up
        }
    }

    fn get_move_history (&self, index: usize) -> f64 {
        match index {
            0 => { return self.user_information.move_histories.total },
            1 => { return self.user_information.move_histories.rock }, 
            2 => { return self.user_information.move_histories.paper },
            3 => { return self.user_information.move_histories.scissors },
            _ => { panic!("you input something wrong in get_move_history stupid") } // if you are reviewing my code this makes me giggle if i mess up
        }
    }

    fn print_move_history (&self) {
        // yikes
        println!("--- User move histories ---\nTotal moves -> {} Rock -> {} | Paper -> {} | Scissors - > {}", self.get_move_history(0), self.get_move_history(1), self.get_move_history(2), self.get_move_history(3));
    }

    fn print_user_win_rate (&self) {
        if self.get_user_win_rate(0) >= 0.50 {
            // yikes
            println!("--- User win rates ---\nOverall -> {} | Rock -> {} | Paper -> {} | Scissors -> {}", self.get_user_win_rate(0).to_string().green(), self.get_user_win_rate(1), self.get_user_win_rate(2), self.get_user_win_rate(3))
        }
        else {
            // yikes
            println!("--- User win rates ---\nOverall -> {} | Rock -> {} | Paper -> {} | Scissors -> {}", self.get_user_win_rate(0).to_string().red(), self.get_user_win_rate(1), self.get_user_win_rate(2), self.get_user_win_rate(3))
        }
       
    }

    // gets user input, returns trimmed user input with length, none is a simple error
    fn get_user_input (&self) -> Option<&str> {
        println!("Enter your first move against your opponent -> rock | paper | scissors (not case sensitive)");

        // stores input once assigned in input_size
        let mut user_input: String = String::new();
        let input_size: usize = std::io::stdin() // read input
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // remove whitespaces and other unnecessary stuff, please let me know if there is a better way to do this
        let user_input: String = user_input.replace(" ", "",).replace("\n", "").replace("\r", "").to_lowercase();

        // if input is empty or isn't rock paper scissors lol
        if user_input == "" || (user_input != String::from("rock") && user_input != String::from("paper") && user_input != String::from("scissors")) { 
            if self.is_debugging {println!("String entered was invalid... -> {}", user_input)}
            return None;
        }
        
        match user_input.as_str() {
            "rock" => return Some("rock"),
            "paper" => return Some("paper"),
            "scissors" => return Some("scissors"),
            _ => panic!("This should never happen, debug get_user_input function.")
        }
    }

}