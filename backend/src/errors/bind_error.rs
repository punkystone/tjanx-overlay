use std::io::Error;

pub struct BindError {
    pub inner: Error,
}

impl From<Error> for BindError {
    fn from(inner: Error) -> Self {
        Self { inner }
    }
}

impl std::fmt::Display for BindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
