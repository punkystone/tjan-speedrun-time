use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};
use tokio::sync::Mutex;

use crate::{
    env::Env, model::validate_request::ValidateRequest, twitch_repository::TwitchRepository,
};

#[get("/validate")]
async fn validate(
    twitch_response: Query<ValidateRequest>,
    twitch_repository: Data<Mutex<TwitchRepository>>,
    env: Data<Env>,
) -> HttpResponse {
    if let (Some(code), Some(_)) = (&twitch_response.code, &twitch_response.scope) {
        let mut twitch_repository = twitch_repository.lock().await;
        match twitch_repository.set_token(code).await {
            Ok(_) => {
                println!("Validated");
                match twitch_repository.set_user_id(&env.channel).await {
                    Ok(_) => {
                        println!("New User ID: {:?}", twitch_repository.user_id);
                    }
                    Err(error) => println!("Get User ID Error: {error:?}"),
                }
            }
            Err(error) => println!("Validation Error: {error:?}"),
        }
    } else if twitch_response.error.is_some() && twitch_response.error_description.is_some() {
        println!(
            "Error: {:?}, Description: {:?} ",
            twitch_response.error, twitch_response.error_description
        );
    }
    HttpResponse::Ok().finish()
}
