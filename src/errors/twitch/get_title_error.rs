use twitch_api2::helix::ClientRequestError;

use super::{get_user_id_error::GetUserIdError, token_error::TokenError};

#[derive(Debug)]
pub enum GetTitleError {
    TokenError(TokenError),
    UserIdError(GetUserIdError),
    UserNotFoundError,
    RequestError,
}

impl std::fmt::Display for GetTitleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GetTitleError::TokenError(e) => write!(f, "{}", e),
            GetTitleError::UserIdError(e) => write!(f, "{}", e),
            GetTitleError::UserNotFoundError => write!(f, "User not found"),
            GetTitleError::RequestError => write!(f, "Twitch Api Request Error"),
        }
    }
}

impl From<GetUserIdError> for GetTitleError {
    fn from(e: GetUserIdError) -> Self {
        GetTitleError::UserIdError(e)
    }
}

impl From<ClientRequestError<reqwest::Error>> for GetTitleError {
    fn from(_: ClientRequestError<reqwest::Error>) -> Self {
        GetTitleError::RequestError
    }
}
