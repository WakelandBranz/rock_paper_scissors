// thank you https://app.quicktype.io/

use serde::{Serialize, Deserialize};
/*
UserInformation contains the following information:
move_histories -> 0, 1, 2 = rock, paper, scissors.  contains amount of each move made by user
user_win_rate -> 0, 1, 2, 3 = overall, rock, paper scissors.  contains the percent of games won based on total, then each move made by user
 */
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct UserInformation {
    // rust structs use snake case, json uses upper CamelCase, therefore we must explicitly name with rename_all
    pub user_win_rate: UserWinRate,
    pub user_wins: UserWins,
    pub move_histories: MoveHistories,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct UserWinRate {
    pub overall: f64,
    pub rock: f64,
    pub paper: f64,
    pub scissors: f64,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct UserWins {
    pub total: f64,
    pub rock: f64,
    pub paper: f64,
    //#[serde(rename = "scissors_")]
    pub scissors: f64,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct MoveHistories {
    pub total: f64,
    pub rock: f64,
    pub paper: f64,
    #[serde(rename = "scissors_")]
    pub scissors: f64,
}

pub struct Moves {
    pub rock: &'static str,
    pub paper: &'static str,
    pub scissors: &'static str,
  }
