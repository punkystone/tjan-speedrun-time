use actix_web::{get, HttpResponse, Responder, web::Data};

use crate::env::Env;

#[get("/auth")]
async fn auth(env: Data<Env> ) -> impl Responder {
    HttpResponse::MovedPermanently()
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
