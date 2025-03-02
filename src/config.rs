use dotenv::dotenv;
use std::env;
use crate::models::{WeatherData, WeatherError};

pub struct WeatherApi {
    api_key: String,
    base_url: String,
}

impl WeatherApi {
    pub fn new() -> Result<WeatherApi, WeatherError> {
        dotenv().ok();
        let api_key = env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY must be set");
        let base_url = env::var("WEATHER_API_URL").expect("WEATHER_API_URL must be set");

        ok(WeatherApi { api_key, base_url })
}