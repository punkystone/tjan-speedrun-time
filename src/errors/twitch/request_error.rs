use std::error::Error;

#[derive(Debug)]
pub struct RequestError {
    pub inner: hyper::Error,
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error executing request")
    }
}

impl Error for RequestError {}

impl From<hyper::Error> for RequestError {
    fn from(error: hyper::Error) -> Self {
        Self { inner: error }
    }
}
