use reqwest::Error;
use serde::{Deserialize,Serialize};

#[derive(Deserialize, Debug)]
struct Response {
    status: String,
    lat: f64,
    lon: f64,
    city: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "generationtime_ms")]
    pub generationtime_ms: f64,
    #[serde(rename = "utc_offset_seconds")]
    pub utc_offset_seconds: i64,
    pub timezone: String,
    #[serde(rename = "timezone_abbreviation")]
    pub timezone_abbreviation: String,
    pub elevation: f64,
    #[serde(rename = "current_units")]
    pub current_units: CurrentUnits,
    pub current: Current,
    #[serde(rename = "daily_units")]
    pub daily_units: DailyUnits,
    pub daily: Daily,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUnits {
    pub time: String,
    pub interval: String,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    pub time: String,
    pub interval: i64,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyUnits {
    pub time: String,
    #[serde(rename = "temperature_2m_max")]
    pub temperature_2m_max: String,
    #[serde(rename = "temperature_2m_min")]
    pub temperature_2m_min: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daily {
    pub time: Vec<String>,
    #[serde(rename = "temperature_2m_max")]
    pub temperature_2m_max: Vec<f64>,
    #[serde(rename = "temperature_2m_min")]
    pub temperature_2m_min: Vec<f64>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ip_address = reqwest::get("https://api.ipify.org").await?.text().await?;
    println!("Your IP Address: {}", ip_address);
    let geolocation = reqwest::get(format!("http://ip-api.com/json/{ip_address}?fields=status,regionName,city,lat,lon")).await?.json::<Response>().await?;
    if geolocation.status == "success" {
        println!("Your are in {:?}.",geolocation.city);
        println!("Your lat&lot: {:?},{:?}",geolocation.lat,geolocation.lon);
        let lat = geolocation.lat;
        let lon = geolocation.lon;
        let weather = reqwest::get(format!("https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&current=temperature_2m&daily=temperature_2m_max,temperature_2m_min&timezone=auto")).await?.json::<WeatherResponse>().await?;
        let current_temp = weather.current.temperature_2m;
        println!("Your city temperature is {:?}C",current_temp);
    }
    
    Ok(())
}