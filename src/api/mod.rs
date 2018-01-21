use rocket::response::Content;
use rocket::http::ContentType;

pub mod debugging;
pub mod registration;
pub mod account;

#[get("/")]
fn hello() -> Content<&'static str> {
    Content(ContentType::HTML, include_str!("index.html"))
}
