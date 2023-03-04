use tokio::sync::Mutex;

use actix_web::{get, web::Data, HttpResponse};

use crate::repository::{format_time, get_leaderboard};

#[get("/time")]
async fn time(counter: Data<Mutex<usize>>) -> HttpResponse {
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    let counter = *counter.lock().await;
    let mut page = ((counter / 200) as f64).ceil() as usize;
    if counter % 200 != 0 {
        page += 1;
    }
    match get_leaderboard(page).await {
        Ok(leaderboard) => {
            for run in leaderboard.runs {
                if run.place == counter {
                    return HttpResponse::Ok().body(format_time(run.igt));
                }
            }
            HttpResponse::Ok().body("Keine Zeit")
        }
        Err(error) => HttpResponse::Ok().body(error.to_string()),
    }
}
