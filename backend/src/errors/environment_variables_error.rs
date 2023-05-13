use std::{env::VarError, num::ParseIntError};

pub struct EnvironmentVariablesError {
    inner: Option<VarError>,
}

impl From<VarError> for EnvironmentVariablesError {
    fn from(inner: VarError) -> Self {
        Self { inner: Some(inner) }
    }
}

impl std::fmt::Display for EnvironmentVariablesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            Some(inner) => write!(f, "Environment variable error: {inner}"),
            None => write!(f, "Environment variable error"),
        }
    }
}

impl From<ParseIntError> for EnvironmentVariablesError {
    fn from(_: ParseIntError) -> Self {
        Self { inner: None }
    }
}
