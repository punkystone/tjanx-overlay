use super::{bind_error::BindError, environment_variables_error::EnvironmentVariablesError};

pub enum RunError {
    BindError(BindError),
    EnvironmentVariablesError(EnvironmentVariablesError),
}

impl From<BindError> for RunError {
    fn from(inner: BindError) -> Self {
        Self::BindError(inner)
    }
}

impl From<EnvironmentVariablesError> for RunError {
    fn from(inner: EnvironmentVariablesError) -> Self {
        Self::EnvironmentVariablesError(inner)
    }
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::BindError(inner) => write!(f, "{inner}"),
            RunError::EnvironmentVariablesError(inner) => {
                write!(f, "{inner}")
            }
        }
    }
}
