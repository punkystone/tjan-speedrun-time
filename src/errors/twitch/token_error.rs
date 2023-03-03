#[derive(Debug)]
pub struct TokenError;

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Token not initialized")
    }
}
