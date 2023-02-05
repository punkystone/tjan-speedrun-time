use serde::Deserialize;

use super::leader_board::LeaderBoard;

#[derive(Deserialize, Debug)]
pub struct LeaderBoardRequest {
    pub leaderboard: LeaderBoard,
}
