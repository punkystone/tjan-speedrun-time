use actix_web::{post, web::Data, HttpRequest, HttpResponse};
use tokio::sync::Mutex;

use crate::{env::Env, twitch_repository::TwitchRepository};

#[post("/place/decrement")]
async fn decrement_place(
    request: HttpRequest,
    counter: Data<Mutex<usize>>,
    twitch_repository: Data<Mutex<TwitchRepository>>,
    env: Data<Env>,
) -> HttpResponse {
    let api_key = request.headers().get("API-KEY");
    if api_key.is_none() {
        return HttpResponse::Unauthorized().body("API Key Missing");
    }
    let api_key = api_key.unwrap();
    let api_key = api_key.to_str();
    if api_key.is_err() {
        return HttpResponse::Unauthorized().body("API Key Invalid");
    }
    let api_key = api_key.unwrap();
    if api_key != env.api_key {
        return HttpResponse::Unauthorized().body("API Key Invalid");
    }
    let mut repository = twitch_repository.lock().await;
    let old_value = *counter.lock().await;
    *counter.lock().await -= 1;
    match repository.get_title().await {
        Ok(Some(title)) => {
            if let Err(error) = repository
                .set_title(title.replace(&old_value.to_string(), &counter.lock().await.to_string()))
                .await
            {
                println!("Set Title Error: {error:?}");
            }
        }
        Ok(None) => println!("Could not get title"),
        Err(error) => println!("Get Title Error: {error:?}"),
    }
    HttpResponse::Ok().finish()
}
