use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    pub location: <Location>,

    pub weatherdata: <Current>,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub tz_id: String,
    pub localtime_epoch: i64,
    pub localtime: String,
}

#[derive(Deserialize, Debug)]
pub struct Current {
    pub last_updated_epoch: i64,
    pub last_updated: String,
    pub temp_c: f64,
    pub temp_f: f64,
    pub is_day: i64,
    pub condition: <Condition>,
    pub wind_mph: f64,
    pub wind_kph: f64,
    pub wind_degree: i64,
    pub wind_dir: String,
    pub pressure_mb: f64,
    pub pressure_in: f64,
    pub precip_mm: f64,
    pub precip_in: f64,
    pub humidity: i64,
    pub cloud: i64,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
    pub vis_km: f64,
    pub vis_miles: f64,
    pub uv: f64,
    pub gust_mph: f64,
    pub gust_kph: f64,
}   

#[derive(Debug)]
pub enum WeatherError {
    RequestFailed(String),
    ParseError(String),
    ConfigError(String),
    CityNotFound,
    Other(String),
}

impl std::fmt::Display for WeatherError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WeatherError::RequestFailed(msg) => write!(f, "API request failed: {}", msg),
            WeatherError::ParseError(msg) => write!(f, "Failed to parse response: {}", msg),
            WeatherError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            WeatherError::CityNotFound => write!(f, "City not found"),
            WeatherError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for WeatherError {}