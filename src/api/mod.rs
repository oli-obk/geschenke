use rocket::response::Content;
use rocket::http::ContentType;

pub mod debugging;
pub mod registration;

#[get("/")]
fn hello() -> Content<&'static str> {
    Content(ContentType::HTML, include_str!("index.html"))
}
