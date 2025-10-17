use thiserror::Error;

/// Custom error types for our weather application
///
/// Using thiserror to automatically implement the Error trait
#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("API request failed: {0}")]
    ApiError(String),

    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("City not found: {0}")]
    CityNotFound(String),
}
