use actix_web::error::HttpError;

use super::{build_request_error::BuildRequestError, request_error::RequestError};

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum SetTitleError {
    MissingUserIdError,
    MissingTokenError,
    BuildRequestError(BuildRequestError),
    RequestError(RequestError),
    UnauthorizedError,
    UnknownTwitchResponseError,
}

impl From<HttpError> for SetTitleError {
    fn from(error: HttpError) -> Self {
        Self::BuildRequestError(error.into())
    }
}

impl From<hyper::Error> for SetTitleError {
    fn from(error: hyper::Error) -> Self {
        Self::RequestError(error.into())
    }
}
