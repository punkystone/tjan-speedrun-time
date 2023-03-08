use std::string::FromUtf8Error;

use actix_web::http::uri::InvalidUri;

pub enum GetLeaderboardError {
    InvalidUrl,
    RequestError,
    Utf8Error,
    ParseError,
}

impl From<InvalidUri> for GetLeaderboardError {
    fn from(_: InvalidUri) -> Self {
        GetLeaderboardError::InvalidUrl
    }
}

impl From<hyper::Error> for GetLeaderboardError {
    fn from(_: hyper::Error) -> Self {
        GetLeaderboardError::RequestError
    }
}

impl From<FromUtf8Error> for GetLeaderboardError {
    fn from(_: FromUtf8Error) -> Self {
        GetLeaderboardError::Utf8Error
    }
}

impl From<serde_json::Error> for GetLeaderboardError {
    fn from(_: serde_json::Error) -> Self {
        GetLeaderboardError::ParseError
    }
}

impl std::fmt::Display for GetLeaderboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetLeaderboardError::InvalidUrl => write!(f, "speedrun.com invalid request url"),
            GetLeaderboardError::RequestError => write!(f, "speedrun.com request error"),
            GetLeaderboardError::Utf8Error => write!(f, "utf8 parse error"),
            GetLeaderboardError::ParseError => write!(f, "speedrun.com response parse error"),
        }
    }
}
