use std::{
    fmt::{Display, Formatter},
    string::FromUtf8Error,
};

use twitch_api2::{client::CompatError, twitch_oauth2::tokens::errors::ValidationError};
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum GetTokenError {
    BuildRequestError,
    RequestError,
    Utf8Error,
    TokenValidationError,
    ParseError,
}

impl Display for GetTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BuildRequestError => write!(f, "Error building request"),
            Self::RequestError => write!(f, "Error sending request"),
            Self::Utf8Error => write!(f, "Utf8 error"),
            Self::TokenValidationError => write!(f, "Error validating token"),
            Self::ParseError => write!(f, "Error parsing response"),
        }
    }
}

impl From<actix_web::error::HttpError> for GetTokenError {
    fn from(_: actix_web::error::HttpError) -> Self {
        Self::BuildRequestError
    }
}

impl From<hyper::Error> for GetTokenError {
    fn from(_: hyper::Error) -> Self {
        Self::RequestError
    }
}

impl From<FromUtf8Error> for GetTokenError {
    fn from(_: FromUtf8Error) -> Self {
        Self::Utf8Error
    }
}

impl From<serde_json::Error> for GetTokenError {
    fn from(_: serde_json::Error) -> Self {
        Self::ParseError
    }
}

impl From<ValidationError<CompatError<reqwest::Error>>> for GetTokenError {
    fn from(_: ValidationError<CompatError<reqwest::Error>>) -> Self {
        Self::TokenValidationError
    }
}
