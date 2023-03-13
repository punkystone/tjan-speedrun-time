pub mod env;
mod errors;

mod middlewares;
mod model;
mod repository;
mod routes;
mod twitch_repository;

use actix_cors::Cors;
use actix_web::{
    middleware::{self},
    web::{Data, QueryConfig},
    App, HttpServer,
};
use env::Env;
use errors::{place_query_error::PlaceQueryError, twitch::validation_error::ValidationError};

use middlewares::api_key_service::ApiKeyService;
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
                env.client_id.clone(),
                env.client_secret.clone(),
                env.redirect_uri.clone(),
                env.access_token.clone(),
                env.refresh_token.clone(),
            )));
            {
                let mut twitch_repository = twitch_repository.lock().await;
                match twitch_repository.validate().await {
                    Ok(valid) => {
                        if valid {
                            println!("Initial Token valid");
                        } else {
                            println!("Initial Token invalid");
                            match twitch_repository.refresh_token().await {
                                Ok(_) => println!("Token refreshed"),
                                Err(e) => println!("Token refresh failed: {e:?}"),
                            }
                        }
                    }
                    Err(ValidationError::NoTokenError) => println!("No Token supplied"),
                    Err(e) => println!("Token validation Failed: {e:?}"),
                }

                match twitch_repository.set_user_id(&env.channel).await {
                    Ok(_) => {
                        println!(
                            "User id set for {}: {:?}",
                            env.channel, twitch_repository.user_id
                        );
                    }
                    Err(e) => println!("Failed to get user id: {e:?}"),
                }
            }

            let counter = Data::new(Mutex::<usize>::new(env.counter));
            let port = env.port;
            let env = Data::new(env);
            HttpServer::new(move || {
                App::new()
                    .app_data(QueryConfig::default().error_handler(|_, _| PlaceQueryError.into()))
                    .wrap(Cors::permissive())
                    .wrap(ApiKeyService {
                        api_key: env.api_key.clone(),
                    })
                    .wrap(
                        middleware::DefaultHeaders::new()
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
