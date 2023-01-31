use std::fmt::{Display, Formatter};

use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub struct PlaceQueryError;

impl Display for PlaceQueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "place query error")
    }
}

impl ResponseError for PlaceQueryError {
    fn status_code(&self) -> hyper::StatusCode {
        hyper::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().body("place query error")
    }
}
