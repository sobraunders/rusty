use rusqlite::{Connection, params};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    openweather_api_key: String,
}

fn main() {
    match load_config() {
        Ok(config) => match set_api_key(&config.openweather_api_key) {
            Ok(_) => println!("✓ API key successfully stored in the database"),
            Err(e) => eprintln!("✗ Error: {}", e),
        },
        Err(e) => eprintln!("✗ Configuration error: {}", e),
    }
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = "config.yaml";
    
    let content = fs::read_to_string(config_path)
        .map_err(|_| format!("config.yaml not found. Please copy config.yaml.example to config.yaml and fill in your API key"))?;
    
    let config: Config = serde_yaml::from_str(&content)?;
    
    if config.openweather_api_key == "your_api_key_here" {
        return Err("Please update config.yaml with your actual API key".into());
    }
    
    Ok(config)
}

fn set_api_key(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("bot_data.db")?;
    
    // Create table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users_data (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            data_key TEXT NOT NULL,
            data_value TEXT NOT NULL,
            UNIQUE(username, data_key)
        )",
        [],
    )?;

    // Insert or replace the API key
    conn.execute(
        "INSERT OR REPLACE INTO users_data (username, data_key, data_value)
         VALUES (?, ?, ?)",
        params!["bot", "openweather_api_key", api_key],
    )?;
    
    Ok(())
}
