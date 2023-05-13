use crate::errors::environment_variables_error::EnvironmentVariablesError;

pub struct Env {
    pub port: u16,
    pub api_key: String,
    pub fetch_interval: u64,
}
impl Env {
    #[allow(clippy::missing_errors_doc)]
    pub fn check_variables() -> Result<Env, EnvironmentVariablesError> {
        Ok(Env {
            port: std::env::var("PORT")?.parse::<u16>()?,
            api_key: std::env::var("API_KEY")?,
            fetch_interval: std::env::var("FETCH_INTERVAL")?.parse::<u64>()?,
        })
    }
}
