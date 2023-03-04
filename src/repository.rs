use std::time::Duration;

use base64::{engine::general_purpose, Engine};
use hyper::{body::to_bytes, Client, Uri};
use hyper_tls::HttpsConnector;

use crate::{
    errors::get_leaderboard_error::GetLeaderboardError,
    model::{leader_board::LeaderBoard, leaderboard_request::LeaderBoardRequest},
};

pub async fn get_leaderboard(page: usize) -> Result<LeaderBoard, GetLeaderboardError> {
    let request = format!(
        "{{\"params\":{{\"gameId\":\"j1npme6p\",\"categoryId\":\"mkeyl926\",\"values\":[{{\"variableId\":\"r8rg67rn\",\"valueIds\":[\"21d4zvp1\"]}},{{\"variableId\":\"wl33kewl\",\"valueIds\":[\"4qye4731\"]}}],\"timer\":2,\"regionIds\":[],\"platformIds\":[],\"video\":0,\"obsolete\":0}},\"page\":{page},\"vary\":1674993674}}",
    );
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

    let uri = format!(
        "https://www.speedrun.com/api/v2/GetGameLeaderboard?_r={}",
        general_purpose::STANDARD_NO_PAD.encode(request)
    )
    .parse::<Uri>()?;
    let response = client.get(uri).await?;

    let response_string = String::from_utf8(to_bytes(response.into_body()).await?.to_vec())?;

    let parsed_response = serde_json::from_str::<LeaderBoardRequest>(&response_string)?;

    Ok(parsed_response.leaderboard)
}

pub fn format_time(igt: f64) -> String {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let duration = Duration::from_millis((igt * 1000.0) as u64);
    let milliseconds = duration.as_millis() % 1000;
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    format!("{minutes}m {seconds}s {milliseconds}ms")
}
