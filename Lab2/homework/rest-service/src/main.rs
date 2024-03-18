use rocket::{launch, routes, get, post, form::Form};
use maud::{Markup, html};

pub mod requests;
pub mod responses;
pub mod models;
pub mod components;

use requests::Request;
use components::Component;

#[get("/")]
async fn index() -> Markup {
    html! {
        form action="/" method="post" {
            h1 { "Prognoza pogody" }
            h2 { "Na następny tydzień" }
            input type="text" id="city" name="city" placeholder="Miejscowość"; br;
            input type="text" id="country" name="country" placeholder="Kraj"; br;
            input type="submit" value="Sprawdź";
        }
    }
}

#[post("/", data="<input>")]
async fn submit(input: Form<Request<'_>>) -> Markup {
    input
        .process()
        .await
        .render()
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, submit])
}
