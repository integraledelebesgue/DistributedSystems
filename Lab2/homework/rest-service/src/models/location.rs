use serde::Deserialize;
use crate::requests::fetch;

#[derive(Debug, Deserialize)]
pub struct Location {
    pub lon: f32,
    pub lat: f32
}

impl Location {
    const API: &'static str = "https://nominatim.openstreetmap.org/search";

    fn url(city: &str, country: &str) -> String {
        format!(
            "{}?q={},{}&format=json",
            Self::API,
            city,
            country
        )
    }

    pub async fn request(city: &str, country: &str) -> Result<Self, reqwest::Error> {
        fetch::<LocationResponse, Location>(&Self::url(city, country)).await
    }
}

#[derive(Debug, Deserialize)]
struct EncodedLocation {
    lat: String,
    lon: String
}

#[derive(Debug, Deserialize)]
struct LocationResponse(Vec<EncodedLocation>);

impl Into<Location> for LocationResponse {
    fn into(self) -> Location {
        let items = self.0;
        let EncodedLocation { lat, lon } = items
            .into_iter()
            .next()
            .unwrap();

        let lat = lat.parse::<f32>().unwrap();
        let lon = lon.parse::<f32>().unwrap();

        Location { lon, lat }
    }
}