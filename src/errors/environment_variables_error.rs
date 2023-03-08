use std::{
    env::VarError,
    fmt::{Display, Formatter},
    num::ParseIntError,
};

pub struct EnvironmentVariablesError;

impl Display for EnvironmentVariablesError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Missing Environment variables")
    }
}

impl From<VarError> for EnvironmentVariablesError {
    fn from(_: VarError) -> Self {
        EnvironmentVariablesError
    }
}

impl From<ParseIntError> for EnvironmentVariablesError {
    fn from(_: ParseIntError) -> Self {
        EnvironmentVariablesError
    }
}
