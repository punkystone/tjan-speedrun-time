use super::{
    build_request_error::BuildRequestError, request_error::RequestError,
    response_parse_error::ResponseParseError, response_to_string_error::ResponseToStringError,
};

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum SetTokenError {
    BuildRequestError(BuildRequestError),
    RequestError(RequestError),
    ResponseToStringError(ResponseToStringError),
    ResponseParseError(ResponseParseError),
}

impl From<actix_web::error::HttpError> for SetTokenError {
    fn from(error: actix_web::error::HttpError) -> Self {
        Self::BuildRequestError(error.into())
    }
}

impl From<hyper::Error> for SetTokenError {
    fn from(error: hyper::Error) -> Self {
        Self::RequestError(error.into())
    }
}

impl From<ResponseToStringError> for SetTokenError {
    fn from(error: ResponseToStringError) -> Self {
        Self::ResponseToStringError(error)
    }
}

impl From<serde_json::Error> for SetTokenError {
    fn from(error: serde_json::Error) -> Self {
        Self::ResponseParseError(error.into())
    }
}
