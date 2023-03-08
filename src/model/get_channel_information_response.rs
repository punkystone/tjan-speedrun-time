use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetChannelInformationResponse {
    pub data: Vec<ChannelInformation>,
}
#[derive(Deserialize)]
pub struct ChannelInformation {
    pub title: String,
}
