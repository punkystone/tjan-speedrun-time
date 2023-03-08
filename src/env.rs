use crate::errors::environment_variables_error::EnvironmentVariablesError;

#[derive(Clone)]
pub struct Env {
    pub port: u16,
    pub counter: usize,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub channel: String,
}
impl Env {
    #[allow(clippy::missing_errors_doc)]
    pub fn check_variables() -> Result<Env, EnvironmentVariablesError> {
        let access_token = std::env::var("ACCESS_TOKEN")?;
        let refresh_token = std::env::var("REFRESH_TOKEN")?;
        Ok(Env {
            port: std::env::var("PORT")?.parse::<u16>()?,
            counter: std::env::var("COUNTER")?.parse::<usize>()?,
            client_id: std::env::var("CLIENT_ID")?,
            redirect_uri: std::env::var("REDIRECT_URI")?,
            client_secret: std::env::var("CLIENT_SECRET")?,
            access_token: if access_token.chars().count() > 0 {
                Some(access_token)
            } else {
                None
            },
            refresh_token: if refresh_token.chars().count() > 0 {
                Some(refresh_token)
            } else {
                None
            },
            channel: std::env::var("CHANNEL")?,
        })
    }
}
