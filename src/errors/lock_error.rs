use std::sync::{MutexGuard, PoisonError};

use actix_web::ResponseError;
#[derive(Debug)]
pub struct LockError;

impl std::fmt::Display for LockError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lock Error")
    }
}
impl ResponseError for LockError {}

impl From<PoisonError<MutexGuard<'_, usize>>> for LockError {
    fn from(_: PoisonError<MutexGuard<'_, usize>>) -> Self {
        LockError
    }
}
