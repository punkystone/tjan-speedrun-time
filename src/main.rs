mod errors;
mod model;
mod repository;
mod routes;

use actix_web::{middleware, web::QueryConfig, App, HttpServer};
use errors::place_query_error::PlaceQueryError;
use routes::index::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(QueryConfig::default().error_handler(|_, _| PlaceQueryError.into()))
            .wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Origin", "*")))
            .service(index)
    })
    .bind(("0.0.0.0", 5556))?
    .run()
    .await
}
