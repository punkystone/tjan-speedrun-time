use actix_web::{get, web::Data, HttpResponse, Responder};

use crate::env::Env;
#[allow(clippy::unused_async)]
#[get("/auth")]
async fn auth(env: Data<Env>) -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .append_header((
            "Location",
            format!(
                "https://id.twitch.tv/oauth2/authorize?response_type=code&client_id={}&redirect_uri={}&scope=channel%3Amanage%3Abroadcast",
                env.client_id,
                env.redirect_uri
            )
        ))
        .finish()
}
