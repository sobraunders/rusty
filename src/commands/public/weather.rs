use irc::client::prelude::Client;
use crate::message::ParsedMessage;
use crate::database::Database;

const API_KEY_KEY: &str = "openweather_api_key";
const OPENWEATHER_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

pub async fn cmd_weather(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    // Get the API key from the database
    let api_key = match db.get_user_data("bot", API_KEY_KEY) {
        Ok(Some(key)) => key,
        Ok(None) => {
            return client.send_privmsg(&msg.channel, "API key not configured. Please set it with: set bot openweather_api_key YOUR_KEY");
        }
        Err(_) => {
            return client.send_privmsg(&msg.channel, "Error retrieving API key from database");
        }
    };

    let author = match &msg.author {
        Some(author) => author,
        None => {
            return client.send_privmsg(&msg.channel, "Could not determine your username");
        }
    };

    // Get the location from command arguments or stored preference
    let location = if !msg.args.is_empty() {
        let new_location = msg.args.join(" ");
        // Store the location for this user
        if let Err(e) = db.set_user_data(author, "weather_location", &new_location) {
            eprintln!("Failed to store location: {}", e);
        }
        new_location
    } else {
        // Try to get stored location
        match db.get_user_data(author, "weather_location") {
            Ok(Some(stored_location)) => stored_location,
            Ok(None) => {
                return client.send_privmsg(&msg.channel, "No location provided. Usage: !weather <city> or !weather <city>,<country_code>");
            }
            Err(_) => {
                return client.send_privmsg(&msg.channel, "Error retrieving stored location");
            }
        }
    };

    // Make the API request
    match fetch_weather(&api_key, &location).await {
        Ok(weather_info) => {
            client.send_privmsg(&msg.channel, &weather_info)
        }
        Err(e) => {
            eprintln!("Weather API error: {}", e);
            client.send_privmsg(&msg.channel, &format!("Error fetching weather: {}", e))
        }
    }
}

async fn fetch_weather(api_key: &str, location: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(OPENWEATHER_URL)
        .query(&[("q", location), ("appid", api_key), ("units", "metric")])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // Check for API errors
    if response.get("cod").and_then(|v| v.as_str()).unwrap_or("") == "404" {
        return Err("Location not found".into());
    }

    if !response.get("cod").and_then(|v| v.as_str()).unwrap_or("200").starts_with("2") {
        let message = response
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown error");
        return Err(format!("API error: {}", message).into());
    }

    // Extract weather information
    let city = response
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");
    
    let country = response
        .get("sys")
        .and_then(|v| v.get("country"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");
    
    let temp = response
        .get("main")
        .and_then(|v| v.get("temp"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    
    let feels_like = response
        .get("main")
        .and_then(|v| v.get("feels_like"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    
    let humidity = response
        .get("main")
        .and_then(|v| v.get("humidity"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    
    let description = response
        .get("weather")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|v| v.get("description"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");
    
    let wind_speed = response
        .get("wind")
        .and_then(|v| v.get("speed"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    Ok(format!(
        "Weather for {}, {}: {} | Temp: {:.1}°C (feels like {:.1}°C) | Humidity: {}% | Wind: {:.1} m/s",
        city, country, description, temp, feels_like, humidity, wind_speed
    ))
}