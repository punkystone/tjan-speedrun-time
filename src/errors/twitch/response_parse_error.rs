#[derive(Debug)]
pub struct ResponseParseError {
    pub inner: serde_json::Error,
}
impl From<serde_json::Error> for ResponseParseError {
    fn from(error: serde_json::Error) -> Self {
        Self { inner: error }
    }
}
