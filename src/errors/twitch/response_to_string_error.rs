use std::string::FromUtf8Error;
#[derive(Debug)]
pub enum ResponseToStringError {
    HyperError(hyper::Error),
    Utf8Error(FromUtf8Error),
}

impl From<hyper::Error> for ResponseToStringError {
    fn from(error: hyper::Error) -> Self {
        Self::HyperError(error)
    }
}

impl From<FromUtf8Error> for ResponseToStringError {
    fn from(error: FromUtf8Error) -> Self {
        Self::Utf8Error(error)
    }
}
