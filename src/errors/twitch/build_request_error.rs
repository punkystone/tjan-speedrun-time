use actix_web::error::HttpError;
#[derive(Debug)]
pub struct BuildRequestError {
    pub inner: HttpError,
}

impl From<HttpError> for BuildRequestError {
    fn from(error: HttpError) -> Self {
        Self { inner: error }
    }
}
