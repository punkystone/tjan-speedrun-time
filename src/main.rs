pub mod env;
mod errors;
mod model;
mod repository;
mod routes;

use std::sync::Mutex;

use actix_web::{
    middleware::{self},
    web::{Data, QueryConfig},
    App, HttpServer,
};
use env::Env;
use errors::place_query_error::PlaceQueryError;
use routes::{decrement_place::decrement_place, place::place, time::time};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match Env::check_variables() {
        Ok(env) => {
            let counter = Data::new(Mutex::<usize>::new(env.counter));
            HttpServer::new(move || {
                App::new()
                    .app_data(QueryConfig::default().error_handler(|_, _| PlaceQueryError.into()))
                    .wrap(
                        middleware::DefaultHeaders::new().add(("Access-Control-Allow-Origin", "*")),
                    )
                    .app_data(counter.clone())
                    .service(time)
                    .service(place)
                    .service(decrement_place)
            })
            .bind(("0.0.0.0", env.port))?
            .run()
            .await
        }
        Err(e) => Ok(println!("{e}")),
    }
}
