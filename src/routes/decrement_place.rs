use actix_web::{post, web::Data, HttpResponse};
use tokio::sync::Mutex;

use crate::twitch_repository::TwitchRepository;

#[post("/place/decrement")]
async fn decrement_place(
    counter: Data<Mutex<usize>>,
    twitch_repository: Data<Mutex<TwitchRepository<'_>>>,
) -> HttpResponse {
    let repository = twitch_repository.lock().await;
    if let Ok(Some(title)) = repository.get_title().await {
        let old_value = *counter.lock().await;
        *counter.lock().await -= 1;

        if let Err(error) = repository
            .set_title(title.replace(&old_value.to_string(), &counter.lock().await.to_string()))
            .await
        {
            println!("Set Title Error: {error:?}");
        }
    }
    HttpResponse::Ok().finish()
}
