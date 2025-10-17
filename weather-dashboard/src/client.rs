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
    ///
    /// # Returns
    /// * `Result<WeatherData, WeatherError>` - Weather data or an error
    ///
    /// # Example
    /// ```no_run
    /// let client = WeatherClient::new("your_api_key".to_string());
    /// let weather = client.fetch_weather("London").await?;
    /// ```
    pub async fn fetch_weather(&self, city: &str) -> Result<WeatherData, WeatherError> {
        // Build the API URL
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
            city, self.api_key
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
            .json::<OpenWeatherResponse>()
            .await?;

        // Convert API response to our WeatherData format
        Ok(WeatherData {
            temperature: response.main.temp,
            feels_like: response.main.feels_like,
            humidity: response.main.humidity,
            description: response.weather[0].description.clone(),
            wind_speed: response.wind.speed,
            source: "OpenWeatherMap".to_string(),
        })
    }
}
