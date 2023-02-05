use std::sync::Mutex;

use actix_web::{get, web::Data, HttpResponse};

use crate::{
    errors::lock_error::LockError,
    repository::{format_time, get_leaderboard},
};

#[get("/time")]
async fn time(counter: Data<Mutex<usize>>) -> Result<HttpResponse, LockError> {
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    let mut page = ((*counter.lock()? / 200) as f64).ceil() as usize;
    if *counter.lock()? % 200 != 0 {
        page += 1;
    }
    match get_leaderboard(page).await {
        Ok(leaderboard) => {
            for run in leaderboard.runs {
                if run.place == *counter.lock()? {
                    return Ok(HttpResponse::Ok().body(format_time(run.igt)));
                }
            }
            Ok(HttpResponse::Ok().body("Keine Zeit"))
        }
        Err(error) => Ok(HttpResponse::Ok().body(error.to_string())),
    }
}
