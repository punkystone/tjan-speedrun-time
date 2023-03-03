use std::sync::Mutex;

use actix_web::{post, web::Data, HttpResponse};

use crate::{errors::lock_error::LockError, twitch_repository::TwitchRepository};

#[post("/place/decrement")]
async fn decrement_place(
    counter: Data<Mutex<usize>>,
    twitch_repository: Data<Mutex<TwitchRepository<'_>>>,
) -> Result<HttpResponse, LockError> {
    let repository = twitch_repository.lock()?;
    if let Ok(Some(title)) = repository.get_title().await {
        let old_value = *counter.lock()?;
        *counter.lock()? -= 1;

        if let Err(error) = repository
            .set_title(title.replace(&old_value.to_string(), &counter.lock()?.to_string()))
            .await
        {
            println!("Set Title Error: {:?}", error);
        }
    }
    Ok(HttpResponse::Ok().finish())
}
