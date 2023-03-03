use twitch_api2::helix::ClientRequestError;

use super::{get_user_id_error::GetUserIdError, token_error::TokenError};
#[derive(Debug)]
pub enum SetTitleError {
    TokenError(TokenError),
    RequestError,
    UserNotFoundError,
    UserIdError(GetUserIdError),
}

impl std::fmt::Display for SetTitleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SetTitleError::TokenError(e) => write!(f, "{}", e),
            SetTitleError::RequestError => write!(f, "Twitch Api Request Error"),
            SetTitleError::UserNotFoundError => write!(f, "User not found"),
            SetTitleError::UserIdError(e) => write!(f, "{}", e),
        }
    }
}

impl From<ClientRequestError<reqwest::Error>> for SetTitleError {
    fn from(_: ClientRequestError<reqwest::Error>) -> Self {
        SetTitleError::RequestError
    }
}

impl From<GetUserIdError> for SetTitleError {
    fn from(e: GetUserIdError) -> Self {
        SetTitleError::UserIdError(e)
    }
}
