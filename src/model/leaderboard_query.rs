use serde::Deserialize;

#[derive(Deserialize)]
pub struct LeaderBoardQuery {
    pub place: usize,
}
