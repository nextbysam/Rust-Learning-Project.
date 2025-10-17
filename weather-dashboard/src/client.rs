use crate::{error::WeatherError, models::*};

/// HTTP client for fetching weather data
pub struct WeatherClient {
    client: reqwest::Client,
    api_key: String,
}

impl WeatherClient {
    /// Creates a new WeatherClient with the given API key
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    /// Fetches current weather for a city
    ///
    /// # Arguments
    /// * `city` - The city name to fetch weather for
    /// * `units` - Units system: "metric" or "imperial"
    ///
    /// # Returns
    /// * `Result<WeatherData, WeatherError>` - Weather data or an error
    ///
    /// # Example
    /// ```no_run
    /// let client = WeatherClient::new("your_api_key".to_string());
    /// let weather = client.fetch_weather("London", "metric").await?;
    /// ```
    pub async fn fetch_weather(&self, city: &str, units: &str) -> Result<WeatherData, WeatherError> {
        // Build the API URL for WeatherAPI.com
        let url = format!(
            "https://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
            self.api_key, city
        );

        // Make the HTTP request
        let http_response = self
            .client
            .get(&url)
            .send()
            .await?; // Network request (can fail)

        // Check if the request was successful
        if !http_response.status().is_success() {
            let status = http_response.status();
            let error_text = http_response.text().await?;
            return Err(WeatherError::ApiError(format!(
                "API returned status {}: {}",
                status, error_text
            )));
        }

        // Parse the JSON response
        let response = http_response
            .json::<WeatherApiResponse>()
            .await?;

        // Convert API response to our WeatherData format
        // Choose temperature and wind speed based on units
        let (temperature, feels_like, wind_speed) = match units {
            "imperial" => (
                response.current.temp_f,
                response.current.feelslike_f,
                response.current.wind_mph,
            ),
            _ => (
                response.current.temp_c,
                response.current.feelslike_c,
                response.current.wind_kph,
            ),
        };

        Ok(WeatherData {
            temperature,
            feels_like,
            humidity: response.current.humidity,
            description: response.current.condition.text,
            wind_speed,
            source: format!("WeatherAPI.com - {}, {}", response.location.name, response.location.country),
        })
    }
}
