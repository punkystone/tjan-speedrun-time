use std::sync::Mutex;

use actix_web::{post, web::Data, HttpResponse, Responder};

#[post("/place/decrement")]
async fn decrement_place(counter: Data<Mutex<usize>>) -> impl Responder {
    match counter.lock() {
        Ok(mut counter) => {
            *counter -= 1;
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::InternalServerError().body("Lock Error"),
    }
}
