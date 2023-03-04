use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};
use tokio::sync::Mutex;

use crate::{model::validate_request::ValidateRequest, twitch_repository::TwitchRepository};

#[get("/validate")]
async fn validate(
    twitch_response: Query<ValidateRequest>,
    twitch_repository: Data<Mutex<TwitchRepository<'_>>>,
) -> HttpResponse {
    if let (Some(code), Some(_)) = (&twitch_response.code, &twitch_response.scope) {
        if let Err(error) = twitch_repository.lock().await.get_token(code).await {
            println!("Validation Error: {:?}", error);
        }
    } else if twitch_response.error.is_some() && twitch_response.error_description.is_some() {
        println!(
            "Error: {:?}, Description: {:?} ",
            twitch_response.error, twitch_response.error_description
        );
    }
    HttpResponse::Ok().finish()
}
