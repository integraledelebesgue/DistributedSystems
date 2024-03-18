use serde::Deserialize;
use futures::future::join_all;
use crate::{models::location::Location, requests::fetch};

#[derive(Debug, Deserialize)]
pub struct Sun {
    pub sunrise: String,
    pub sunset: String
}

impl Sun {
    const API: &'static str = "https://api.sunrisesunset.io/json";

    fn url(location: &Location, time: &str) -> String {
        format!(
            "{}?lat={}&lng={}&time={}",
            Self::API,
            location.lat,
            location.lon,
            time
        )
    }

    pub async fn request(location: &Location, time: &str) -> Result<Self, reqwest::Error> {
        fetch::<SunResponse, Sun>(&Self::url(location, time)).await
    }

    pub async fn request_many(location: &Location, times: &Vec<String>) -> Result<Vec<Self>, reqwest::Error> {
        let requests = times
            .iter()
            .map(|time| Self::request(&location, &time));

        join_all(requests)
            .await
            .into_iter()
            .collect()
    }
}

#[derive(Debug, Deserialize)]
struct SunResponse {
    results: Sun
}

impl Into<Sun> for SunResponse {
    fn into(self) -> Sun {
        self.results
    }
}