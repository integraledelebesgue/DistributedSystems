use rocket::form::FromForm;
use serde::Deserialize;
use lazy_static::lazy_static;

use crate::{
    models::{
        location::Location,
        sun::Sun,
        forecast::Forecast
    },
    responses::Response
};

const IDENTITY: &'static str = "John's Weather App";

lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::ClientBuilder::new()
        .user_agent(IDENTITY)
        .build()
        .unwrap();
}

pub async fn fetch<Recv, Res>(url: &str) -> Result<Res, reqwest::Error> 
where 
    Recv: for<'a> Deserialize<'a> + Into<Res>, 
    Res: for<'a> Deserialize<'a>
{
    let response = CLIENT
        .get(url)
        .send()
        .await?
        .json::<Recv>()
        .await?;

    Ok(response.into())
}

#[derive(Debug, FromForm)]
pub struct Request<'a> {
    city: &'a str,
    country: &'a str
}

impl<'a> Request<'a> {
    pub async fn process(&self) -> Result<Response, reqwest::Error> {
        let location = Location::request(self.city, self.country).await?;
        let forecast = Forecast::request(&location).await?;
        let sun = Sun::request_many(&location, &forecast.time).await?;

        Ok(Response::new(location, forecast, sun))
    }
}
