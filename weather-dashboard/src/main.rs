// Module declarations - tells Rust these files are part of our crate
mod models;
mod error;
mod client;

use clap::Parser;
use colored::Colorize;
use client::WeatherClient;

/// CLI Weather Dashboard
/// Fetches and displays current weather data
#[derive(Parser, Debug)]
#[command(name = "weather")]
#[command(about = "A CLI weather dashboard", long_about = None)]
struct Cli {
    /// City name to fetch weather for
    city: String,

    /// Units: metric or imperial
    #[arg(short, long, default_value = "metric")]
    units: String,
}

// The #[tokio::main] macro transforms this into:
// fn main() {
//     tokio::runtime::Runtime::new().unwrap().block_on(async {
//         // your code here
//     })
// }
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file (if it exists)
    dotenvy::dotenv().ok();

    // Parse command line arguments
    let cli = Cli::parse();

    // Get API key from environment variable
    let api_key = std::env::var("WEATHER_API_KEY")
        .expect("WEATHER_API_KEY must be set in .env file");

    println!("{}", format!("🌤️  Fetching weather for {}...", cli.city).cyan());

    // Create client and fetch weather
    let client = WeatherClient::new(api_key);
    let weather = client.fetch_weather(&cli.city, &cli.units).await?;

    // Display results with colors!
    let (temp_unit, wind_unit) = if cli.units == "imperial" {
        ("°F", "mph")
    } else {
        ("°C", "km/h")
    };

    println!("\n{}", "Weather Report".bold().underline());
    println!("{}: {}", "City".bold(), cli.city);
    println!("{}: {}{}", "Temperature".bold(), weather.temperature.to_string().yellow(), temp_unit);
    println!("{}: {}{}", "Feels like".bold(), weather.feels_like.to_string().yellow(), temp_unit);
    println!("{}: {}%", "Humidity".bold(), weather.humidity.to_string().blue());
    println!("{}: {}", "Conditions".bold(), weather.description);
    println!("{}: {} {}", "Wind speed".bold(), weather.wind_speed.to_string().green(), wind_unit);
    println!("{}: {}", "Source".bold(), weather.source.dimmed());

    Ok(())
}
