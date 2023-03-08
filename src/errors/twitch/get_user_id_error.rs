use actix_web::error::HttpError;

use super::{
    build_request_error::BuildRequestError, request_error::RequestError,
    response_parse_error::ResponseParseError, response_to_string_error::ResponseToStringError,
};

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum GetUserIdError {
    BuildRequestError(BuildRequestError),
    RequestError(RequestError),
    ResponseToStringError(ResponseToStringError),
    ResponseParseError(ResponseParseError),
    MissingTokenError,
    UnauthorizedError,
    UnknownTwitchResponseError,
}

impl From<HttpError> for GetUserIdError {
    fn from(error: HttpError) -> Self {
        Self::BuildRequestError(error.into())
    }
}

impl From<hyper::Error> for GetUserIdError {
    fn from(error: hyper::Error) -> Self {
        Self::RequestError(error.into())
    }
}

impl From<ResponseToStringError> for GetUserIdError {
    fn from(error: ResponseToStringError) -> Self {
        Self::ResponseToStringError(error)
    }
}

impl From<serde_json::Error> for GetUserIdError {
    fn from(error: serde_json::Error) -> Self {
        Self::ResponseParseError(error.into())
    }
}
