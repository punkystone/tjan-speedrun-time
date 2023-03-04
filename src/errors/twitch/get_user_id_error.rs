use twitch_api2::helix::ClientRequestError;

use super::token_error::TokenError;
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum GetUserIdError {
    TokenError(TokenError),
    RequestError,
}

impl std::fmt::Display for GetUserIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GetUserIdError::TokenError(e) => write!(f, "{e}"),
            GetUserIdError::RequestError => write!(f, "Twitch Api Request Error"),
        }
    }
}

impl From<ClientRequestError<reqwest::Error>> for GetUserIdError {
    fn from(_: ClientRequestError<reqwest::Error>) -> Self {
        GetUserIdError::RequestError
    }
}
