use maud::{Markup, html};

use crate::{
    models::{
        location::Location,
        sun::Sun,
        forecast::Forecast
    },
    components::Component
};

pub struct Response {
    location: Location,
    forecast: Forecast,
    sun: Vec<Sun>
}

impl Response {
    pub fn new(location: Location, forecast: Forecast, sun: Vec<Sun>) -> Self {
        Response { location, forecast, sun }
    }

    fn day(&self, i: usize) -> Markup {
        let time = &self.forecast.time[i];

        let temp_max = &self.forecast.temperature_2m_max[i];
        let temp_min = &self.forecast.temperature_2m_min[i];
        let temperature = format!("Temperatura: {temp_min}°C - {temp_max}°C");

        let rain = &self.forecast.rain_sum[i];
        let rain = format!("Opady: {rain} mm");

        let sunrise = &self.sun[i].sunrise;
        let sunrise = format!("Wschód Słońca: {sunrise}");

        let sunset = &self.sun[i].sunset;
        let sunset = format!("Zachód Słońca: {sunset}");

        html! {
            line { (time) } br;
            line { (temperature) } br;
            line { (rain) } br;
            line { (sunrise) } br;
            line { (sunset) } br;
        }
    }

    fn week(&self) -> Markup {
        html! {
            (self.day(0)) br; br;
            (self.day(1)) br; br;
            (self.day(2)) br; br;
            (self.day(3)) br; br;
            (self.day(4)) br; br;
            (self.day(5)) br; br;
            (self.day(6)) br; br;
        }
    }
}

impl Component for Response {
    fn render(&self) -> Markup {
        html! {
            h1 { "Prognoza pogody" }
            h3 { "Dla lokalizacji " (self.location.lon) ", " (self.location.lat) }
            (self.week())
        }
    }
}