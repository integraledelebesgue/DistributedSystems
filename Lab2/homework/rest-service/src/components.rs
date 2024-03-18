use maud::{Markup, html};

pub trait Component {
    fn render(&self) -> Markup;
}

impl<T: Component, E: Component> Component for Result<T, E> {
    fn render(&self) -> Markup {
        match self {
            Ok(result) => result.render(),
            Err(error) => error.render()
        }
    }
}

impl Component for reqwest::Error {
    fn render(&self) -> Markup {
        html! {
            h1 { "Request error" }
            h2 { (self.to_string()) }
        }
    }
}