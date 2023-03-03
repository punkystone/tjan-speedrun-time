use hyper::{Body, Client as HyperClient, Method, Request as HyperRequest};
use hyper_tls::HttpsConnector;
use twitch_api2::{
    helix::{
        channels::{
            GetChannelInformationRequest, ModifyChannelInformationBody,
            ModifyChannelInformationRequest,
        },
        users::GetUsersRequest,
    },
    twitch_oauth2::{AccessToken, ClientSecret, RefreshToken, UserToken},
    types::{Nickname, UserId},
    HelixClient,
};

use crate::{
    errors::twitch::{
        get_title_error::GetTitleError, get_token_error::GetTokenError,
        get_user_id_error::GetUserIdError, set_title_error::SetTitleError, token_error::TokenError,
    },
    model::oauth_response::OAuthResponse,
};

pub struct TwitchRepository<'a> {
    client: HelixClient<'a, reqwest::Client>,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    token: Option<UserToken>,
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl TwitchRepository<'_> {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        access_token: Option<String>,
        refresh_token: Option<String>,
    ) -> Self {
        Self {
            client: HelixClient::default(),
            client_id,
            client_secret,
            redirect_uri,
            token: None,
            access_token,
            refresh_token,
        }
    }
    pub async fn init_token(&mut self) {
        if let (Some(access_token), Some(refresh_token)) = (&self.access_token, &self.refresh_token)
        {
            self.token = Some(
                UserToken::from_existing(
                    &self.client,
                    AccessToken::new(access_token),
                    RefreshToken::new(refresh_token),
                    ClientSecret::from(self.client_secret.clone()),
                )
                .await
                .unwrap(),
            )
        } else {
            self.token = None
        }
    }

    pub async fn get_token(&mut self, code: &String) -> Result<(), GetTokenError> {
        let request = HyperRequest::builder()
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
        let response = HyperClient::builder()
            .build::<_, hyper::Body>(HttpsConnector::new())
            .request(request)
            .await?;
        let response_string =
            String::from_utf8(hyper::body::to_bytes(response.into_body()).await?.to_vec())?;
        let oauth_token = serde_json::from_str::<OAuthResponse>(&response_string)?;

        println!("Access Token: {:?}", oauth_token.access_token);
        println!("Refresh Token: {:?}", oauth_token.refresh_token);

        self.token = Some(
            UserToken::from_existing(
                &self.client,
                AccessToken::new(oauth_token.access_token),
                RefreshToken::from(oauth_token.refresh_token),
                ClientSecret::from(self.client_secret.clone()),
            )
            .await?,
        );
        Ok(())
    }

    pub async fn get_title(&self) -> Result<Option<String>, GetTitleError> {
        if let Some(token) = &self.token {
            let user_id = self.get_user_id(String::from("tjan")).await?;
            if let Some(user_id) = user_id {
                let request = GetChannelInformationRequest::builder()
                    .broadcaster_id(user_id)
                    .build();

                let response = self.client.req_get(request, token).await?.data;
                if let Some(channel) = response {
                    return Ok(Some(channel.title));
                }
            } else {
                return Err(GetTitleError::UserNotFoundError);
            }
        } else {
            return Err(GetTitleError::TokenError(TokenError));
        }
        Ok(None)
    }

    pub async fn set_title(&self, title: String) -> Result<(), SetTitleError> {
        if let Some(token) = &self.token {
            let user_id = self.get_user_id(String::from("tjan")).await?;

            if let Some(user_id) = user_id {
                let request = ModifyChannelInformationRequest::builder()
                    .broadcaster_id(user_id)
                    .build();

                let body = ModifyChannelInformationBody::builder().title(title).build();

                self.client.req_patch(request, body, token).await?;
            } else {
                return Err(SetTitleError::UserNotFoundError);
            }
        } else {
            return Err(SetTitleError::TokenError(TokenError));
        }
        Ok(())
    }

    async fn get_user_id(&self, username: String) -> Result<Option<UserId>, GetUserIdError> {
        if let Some(token) = &self.token {
            let request = GetUsersRequest::builder()
                .login(vec![Nickname::new(username)])
                .build();
            let response = self.client.req_get(request, token).await?.data;
            if let Some(user) = response.first() {
                return Ok(Some(user.id.clone()));
            }
        } else {
            return Err(GetUserIdError::TokenError(TokenError));
        }
        Ok(None)
    }
}
