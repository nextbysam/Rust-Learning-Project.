use serde::{Deserialize, Serialize};

/// Our unified weather data structure
/// This is what we'll display to the user
#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: u8,
    pub description: String,
    pub wind_speed: f64,
    pub source: String,
}

/// OpenWeatherMap API response structure
/// This matches their JSON format exactly
#[derive(Debug, Deserialize)]
pub struct OpenWeatherResponse {
    pub main: Main,
    pub weather: Vec<Weather>,
    pub wind: Wind,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub humidity: u8,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
}
