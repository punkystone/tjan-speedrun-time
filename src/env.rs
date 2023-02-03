use crate::errors::environment_variables_error::EnvironmentVariablesError;

pub struct Env {
    pub port: u16,
    pub counter: usize,
}
impl Env {
    pub fn check_variables() -> Result<Env, EnvironmentVariablesError> {
        Ok(Env {
            port: std::env::var("PORT")?.parse::<u16>()?,
            counter: std::env::var("COUNTER")?.parse::<usize>()?,
        })
    }
}
