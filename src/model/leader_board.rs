use serde::Deserialize;

use super::run::Run;

#[derive(Deserialize, Debug)]
pub struct LeaderBoard {
    pub runs: Vec<Run>,
}
