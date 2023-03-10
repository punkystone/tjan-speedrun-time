use hyper::{client::HttpConnector, Body, Client, Method, Request, Response, StatusCode};
use hyper_tls::HttpsConnector;

use crate::{
    errors::twitch::{
        get_title_error::GetTitleError, get_user_id_error::GetUserIdError,
        refresh_token_error::RefreshTokenError, response_to_string_error::ResponseToStringError,
        set_title_error::SetTitleError, set_token_error::SetTokenError,
        validation_error::ValidationError,
    },
    model::{
        get_channel_information_response::GetChannelInformationResponse,
        get_users_response::GetUsersResponse, oauth_response::OAuthResponse,
    },
};

pub struct TwitchRepository {
    client: Client<HttpsConnector<HttpConnector>>,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    pub access_token: Option<String>,
    refresh_token: Option<String>,
    pub user_id: Option<String>,
}

impl TwitchRepository {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        access_token: Option<String>,
        refresh_token: Option<String>,
    ) -> Self {
        Self {
            client: Client::builder().build::<_, hyper::Body>(HttpsConnector::new()),
            client_id,
            client_secret,
            redirect_uri,
            access_token,
            refresh_token,
            user_id: None,
        }
    }

    pub async fn validate(&self) -> Result<bool, ValidationError> {
        if let (Some(access_token), Some(_)) = (&self.access_token, &self.refresh_token) {
            let request = Request::builder()
                .method(Method::GET)
                .uri("https://id.twitch.tv/oauth2/validate")
                .header("Authorization", format!("OAuth {access_token}"))
                .body(Body::empty())?;

            let response = self.client.request(request).await?;
            Ok(response.status() == StatusCode::OK)
        } else {
            Err(ValidationError::NoTokenError)
        }
    }

    pub async fn refresh_token(&mut self) -> Result<(), RefreshTokenError> {
        println!("Refreshing Token");
        if let Some(refresh_token) = &self.refresh_token {
            let request = Request::builder()
                .method(Method::POST)
                .uri("https://id.twitch.tv/oauth2/token")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(format!(
                    concat!(
                        "client_id={}&",
                        "client_secret={}&",
                        "grant_type=refresh_token&",
                        "refresh_token={}&",
                    ),
                    self.client_id, self.client_secret, refresh_token
                )))?;

            let response = self.client.request(request).await?;
            let status_code = response.status();
            if status_code == StatusCode::BAD_REQUEST {
                Err(RefreshTokenError::InvalidRefreshTokenError)
            } else if status_code == StatusCode::OK {
                let response_string = Self::response_to_string(response).await?;
                let oauth_token = serde_json::from_str::<OAuthResponse>(&response_string)?;
                println!("Access Token: {:?}", oauth_token.access_token);
                println!("Refresh Token: {:?}", oauth_token.refresh_token);
                self.access_token = Some(oauth_token.access_token);
                self.refresh_token = Some(oauth_token.refresh_token);
                Ok(())
            } else {
                Err(RefreshTokenError::UnknownTwitchResponseError)
            }
        } else {
            Err(RefreshTokenError::MissingRefreshTokenError)
        }
    }

    pub async fn set_token(&mut self, code: &String) -> Result<(), SetTokenError> {
        let request = Request::builder()
            .method(Method::POST)
            .uri("https://id.twitch.tv/oauth2/token")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(format!(
                concat!(
                    "client_id={}&",
                    "client_secret={}&",
                    "code={}&",
                    "grant_type=authorization_code&",
                    "redirect_uri={}"
                ),
                self.client_id, self.client_secret, code, self.redirect_uri
            )))?;

        let response = self.client.request(request).await?;

        let response_string = Self::response_to_string(response).await?;
        let oauth_token = serde_json::from_str::<OAuthResponse>(&response_string)?;

        println!("Access Token: {:?}", oauth_token.access_token);
        println!("Refresh Token: {:?}", oauth_token.refresh_token);

        self.access_token = Some(oauth_token.access_token);
        self.refresh_token = Some(oauth_token.refresh_token);

        Ok(())
    }
    pub async fn get_title(&mut self) -> Result<Option<String>, GetTitleError> {
        let title = self.get_title_raw().await;
        if let Err(GetTitleError::UnauthorizedError) = title {
            if self.refresh_token().await.is_err() {
                return Err(GetTitleError::UnauthorizedError);
            }
            return self.get_title_raw().await;
        }
        title
    }

    async fn get_title_raw(&self) -> Result<Option<String>, GetTitleError> {
        if let Some(user_id) = &self.user_id {
            if let Some(access_token) = &self.access_token {
                let request = Request::builder()
                    .method(Method::GET)
                    .uri(format!(
                        "https://api.twitch.tv/helix/channels?broadcaster_id={user_id}",
                    ))
                    .header("Authorization", format!("Bearer {access_token}",))
                    .header("Client-Id", &self.client_id)
                    .body(Body::empty())?;

                let response = self.client.request(request).await?;
                let status = response.status();
                if status == StatusCode::OK {
                    let response_string = Self::response_to_string(response).await?;
                    let channel_information_response =
                        serde_json::from_str::<GetChannelInformationResponse>(&response_string)?;

                    Ok(channel_information_response
                        .data
                        .first()
                        .map(|channel| channel.title.clone()))
                } else if status == StatusCode::UNAUTHORIZED {
                    Err(GetTitleError::UnauthorizedError)
                } else {
                    Err(GetTitleError::UnknownTwitchResponseError)
                }
            } else {
                Err(GetTitleError::MissingTokenError)
            }
        } else {
            Err(GetTitleError::MissingUserIdError)
        }
    }

    pub async fn set_title(&mut self, title: String) -> Result<(), SetTitleError> {
        let title_response = self.set_title_raw(&title).await;
        if let Err(SetTitleError::UnauthorizedError) = title_response {
            if self.refresh_token().await.is_err() {
                return Err(SetTitleError::UnauthorizedError);
            }
            return self.set_title_raw(&title).await;
        }
        title_response
    }

    async fn set_title_raw(&self, title: &String) -> Result<(), SetTitleError> {
        if let Some(user_id) = &self.user_id {
            if let Some(access_token) = &self.access_token {
                let request = Request::builder()
                    .method(Method::PATCH)
                    .uri(format!(
                        "https://api.twitch.tv/helix/channels?broadcaster_id={user_id}",
                    ))
                    .header("Authorization", format!("Bearer {access_token}"))
                    .header("Client-Id", &self.client_id)
                    .header("Content-Type", "application/json")
                    .body(Body::from(format!("{{\"title\": \"{title}\"}}")))?;

                let response = self.client.request(request).await?;
                let status = response.status();
                if status == StatusCode::NO_CONTENT {
                    Ok(())
                } else if status == StatusCode::UNAUTHORIZED {
                    Err(SetTitleError::UnauthorizedError)
                } else {
                    Err(SetTitleError::UnknownTwitchResponseError)
                }
            } else {
                Err(SetTitleError::MissingTokenError)
            }
        } else {
            Err(SetTitleError::MissingUserIdError)
        }
    }

    pub async fn set_user_id(&mut self, username: &String) -> Result<(), GetUserIdError> {
        if let Some(access_token) = &self.access_token {
            let request = Request::builder()
                .method(Method::GET)
                .uri(format!(
                    "https://api.twitch.tv/helix/users?login={username}",
                ))
                .header("Authorization", format!("Bearer {access_token}"))
                .header("Client-Id", &self.client_id)
                .body(Body::empty())?;

            let response = self.client.request(request).await?;
            let status = response.status();
            if status == StatusCode::OK {
                let response_string = Self::response_to_string(response).await?;

                let user_id = serde_json::from_str::<GetUsersResponse>(&response_string)?;
                self.user_id = user_id.data.first().map(|user| user.id.clone());
                Ok(())
            } else if status == StatusCode::UNAUTHORIZED {
                Err(GetUserIdError::UnauthorizedError)
            } else {
                Err(GetUserIdError::UnknownTwitchResponseError)
            }
        } else {
            Err(GetUserIdError::MissingTokenError)
        }
    }

    async fn response_to_string(response: Response<Body>) -> Result<String, ResponseToStringError> {
        Ok(String::from_utf8(
            hyper::body::to_bytes(response.into_body()).await?.to_vec(),
        )?)
    }
}
