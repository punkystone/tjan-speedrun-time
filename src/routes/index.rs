use actix_web::{get, web::Query, HttpResponse, Responder};

use crate::{
    model::leaderboard_query::LeaderBoardQuery,
    repository::{format_time, get_leaderboard},
};

#[get("/")]
async fn index(query: Query<LeaderBoardQuery>) -> impl Responder {
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    let mut page = ((query.place / 200) as f64).ceil() as usize;
    if query.place % 200 != 0 {
        page += 1;
    }
    match get_leaderboard(page).await {
        Ok(leaderboard) => {
            for run in leaderboard.runs {
                if run.place == query.place {
                    return HttpResponse::Ok().body(format_time(run.igt));
                }
            }
            HttpResponse::NoContent().body("Not Found")
        }
        Err(error) => HttpResponse::Ok().body(error.to_string()),
    }
}
