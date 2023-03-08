use actix_web::error::HttpError;

use super::{build_request_error::BuildRequestError, request_error::RequestError};
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum ValidationError {
    NoTokenError,
    BuildRequestError(BuildRequestError),
    RequestError(RequestError),
}

impl From<HttpError> for ValidationError {
    fn from(error: HttpError) -> Self {
        Self::BuildRequestError(error.into())
    }
}

impl From<hyper::Error> for ValidationError {
    fn from(error: hyper::Error) -> Self {
        Self::RequestError(error.into())
    }
}
