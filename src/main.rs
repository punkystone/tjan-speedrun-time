pub mod env;
mod errors;
mod model;
mod repository;
mod routes;
mod twitch_repository;

use actix_web::{
    middleware::{self},
    web::{Data, QueryConfig},
    App, HttpServer,
};
use env::Env;
use errors::place_query_error::PlaceQueryError;
use routes::{
    auth::auth, decrement_place::decrement_place, place::place, time::time, validate::validate,
};
use tokio::sync::Mutex;
use twitch_repository::TwitchRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match Env::check_variables() {
        Ok(env) => {
            let twitch_repository = Data::new(Mutex::new(TwitchRepository::new(
                env.client_id.to_owned(),
                env.client_secret.to_owned(),
                env.redirect_uri.to_owned(),
                env.access_token.to_owned(),
                env.refresh_token.to_owned(),
                env.channel.to_owned(),
            )));
            if let Err(error) = twitch_repository.lock().await.init_token().await {
                println!("{error}");
            }
            let counter = Data::new(Mutex::<usize>::new(env.counter));
            let port = env.port;
            let env = Data::new(env);
            HttpServer::new(move || {
                App::new()
                    .app_data(QueryConfig::default().error_handler(|_, _| PlaceQueryError.into()))
                    .wrap(
                        middleware::DefaultHeaders::new()
                            .add(("Access-Control-Allow-Origin", "*"))
                            .add(("Cache-Control", "no-cache, no-store, must-revalidate"))
                            .add(("Pragma", "no-cache"))
                            .add(("Expires", "0")),
                    )
                    .app_data(counter.clone())
                    .app_data(env.clone())
                    .app_data(twitch_repository.clone())
                    .service(time)
                    .service(place)
                    .service(decrement_place)
                    .service(auth)
                    .service(validate)
            })
            .bind(("0.0.0.0", port))?
            .run()
            .await
        }
        Err(e) => Ok(println!("{e}")),
    }
}
