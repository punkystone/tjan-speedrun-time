use tokio::sync::Mutex;

use actix_web::{get, web::Data, HttpResponse, Responder};

#[get("/place")]
async fn place(counter: Data<Mutex<usize>>) -> impl Responder {
    HttpResponse::Ok().body(counter.lock().await.to_string())
}
