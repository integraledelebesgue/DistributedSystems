use serde::Deserialize;
use crate::{models::location::Location, requests::fetch};

#[derive(Debug, Deserialize)]
pub struct Forecast {
    pub time: Vec<String>,
    pub temperature_2m_max: Vec<f32>,
    pub temperature_2m_min: Vec<f32>,
    pub rain_sum: Vec<f32>
}

impl Forecast {
    const API: &'static str = "https://api.open-meteo.com/v1/forecast";

    fn url(location: &Location) -> String {
        format!(
            "{}?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,rain_sum",
            Self::API,
            location.lat,
            location.lon
        )
    }

    pub async fn request(location: &Location) -> Result<Self, reqwest::Error> {
        fetch::<ForecastResponse, Forecast>(&Self::url(location)).await
    }
}

#[derive(Debug, Deserialize)]
struct ForecastResponse {
    daily: Forecast
}

impl Into<Forecast> for ForecastResponse {
    fn into(self) -> Forecast {
        self.daily
    }
}
