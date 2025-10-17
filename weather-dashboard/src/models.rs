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

/// WeatherAPI.com response structure
/// This matches their JSON format exactly
#[derive(Debug, Deserialize)]
pub struct WeatherApiResponse {
    pub location: Location,
    pub current: Current,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct Current {
    pub temp_c: f64,
    pub temp_f: f64,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
    pub humidity: u8,
    pub condition: Condition,
    pub wind_kph: f64,
    pub wind_mph: f64,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub text: String,
}
