use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetUsersResponse {
    pub data: Vec<User>,
}

#[derive(Deserialize)]
pub struct User {
    pub id: String,
}
