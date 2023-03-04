use twitch_api2::{client::CompatError, twitch_oauth2::tokens::errors::ValidationError};

pub struct InitTokenError;

impl std::fmt::Display for InitTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Init Token Error")
    }
}

impl From<ValidationError<CompatError<reqwest::Error>>> for InitTokenError {
    fn from(_: ValidationError<CompatError<reqwest::Error>>) -> Self {
        Self
    }
}
