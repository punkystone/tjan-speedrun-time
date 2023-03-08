use actix_web::{post, web::Data, HttpResponse};
use tokio::sync::Mutex;

use crate::twitch_repository::TwitchRepository;

#[post("/place/decrement")]
async fn decrement_place(
    counter: Data<Mutex<usize>>,
    twitch_repository: Data<Mutex<TwitchRepository>>,
) -> HttpResponse {
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
