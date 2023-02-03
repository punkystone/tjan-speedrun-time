use std::sync::Mutex;

use actix_web::{get, web::Data, HttpResponse, Responder};

#[get("/place")]
async fn place(counter: Data<Mutex<usize>>) -> impl Responder {
    match counter.lock() {
        Ok(counter) => HttpResponse::Ok().body(counter.to_string()),
        Err(_) => HttpResponse::InternalServerError().body("Lock Error"),
    }
}
