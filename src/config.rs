use dotenv::dotenv;
use std::env;
use crate::models::{WeatherData, WeatherError};
use reqwest::Client;

pub struct WeatherApi {
    api_key: String,
    base_url: String,
    client: Client,
}

impl WeatherApi {
    pub fn new() -> Self {

        //load environment variables
        dotenv().ok();
        let api_key = env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY must be set");
        let base_url = env::var("WEATHER_API_URL").expect("WEATHER_API_URL must be set");

        //make a new client
        let client = Client::new();

        WeatherApi { api_key, base_url, client }
}
    //fetch current weather for a location
    pub async fn get_current_weather(&self, city: &str) -> Result<WeatherData, WeatherError> {
        let url = format!("{}/current.json?key={}&q={}&aqi=no", 
            self.base_url, 
            self.api_key, 
            city);

        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|_| WeatherError::RequestFailed("Request failed".to_string()))?;

        match response {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    let data = response.json::<WeatherData>().await;
                    match data {
                        Ok(data) => Ok(data),
                        Err(_) => Err(WeatherError::ParseError("Failed to parse response".to_string())),
                    }
                } else {
                    Err(WeatherError::RequestFailed(format!("Request failed with status code: {}", status)))
                }
            }
            Err(_) => Err(WeatherError::RequestFailed("Request failed".to_string())),
        }
        reponse.json::<WeatherData>()
        .await
        map_err(|_| WeatherError::ParseError("Request failed".to_string()))?;
    }

