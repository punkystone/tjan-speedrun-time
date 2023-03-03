use std::sync::{MutexGuard, PoisonError};

use actix_web::ResponseError;

use crate::twitch_repository::TwitchRepository;
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
impl From<PoisonError<MutexGuard<'_, TwitchRepository<'_>>>> for LockError {
    fn from(_: PoisonError<MutexGuard<'_, TwitchRepository<'_>>>) -> Self {
        LockError
    }
}
