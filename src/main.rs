#![feature(plugin)]
#![feature(custom_derive)]
#![feature(entry_or_default)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate geschenke;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate email_format;
extern crate mailstrom;
extern crate rand;

extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate horrorshow;
extern crate accept_language;
extern crate fluent;
#[macro_use]
extern crate maplit;
extern crate asciifier;

use rocket::Request;

#[macro_use]
mod ui;
mod api;
mod mail;
mod pool;

fn main() {
    let rocket = rocket::ignite()
        .mount("/", routes![
            api::hello,
            fuchur,
        ])
        .mount(
            "/registration",
            routes![api::registration::create_user_form,],
        )
        .mount(
            "/account",
            routes![
                api::account::logout,
                api::account::login,
                api::account::login_key,
                api::account::recover,
            ],
        )
        .mount(
            "/present",
            routes![
                api::present::edit,
                api::present::view,
                api::present::add,
                api::present::delete,
                api::present::gift,
                api::present::free,
            ],
        )
        .mount(
            "/user",
            routes![
                api::user::add_friend,
                api::user::custom_add_friend,
                api::user::custom_add_friend_apply,
                api::user::remove_friend,
                api::user::view,
            ],
        )
        .catch(errors![not_found, bad_parse,]);
    let rocket = if option_env!("ROCKET_ENV").unwrap_or("development") == "development" {
        rocket.mount(
            "/debugging",
            routes![
                api::debugging::get_presents,
                api::debugging::get_users,
                api::debugging::user_info,
            ],
        )
    } else {
        rocket
    };
    rocket
        .manage(pool::establish_connection())
        .manage(mail::init())
        .manage(ui::localization::load())
        .launch();
}

use rocket::http::ContentType;
use rocket::response::Content;

#[error(404)]
fn not_found(req: &Request) -> Content<String> {
    Content(ContentType::HTML, format!("<h1>404 Brain Not Found</h1><img src=\"/fuchur.jpg\"/><br/><pre>{:#?}</pre>", req))
}

#[error(422)]
fn bad_parse(req: &Request) -> String {
    format!("422:<br/>\n{:#?}", req)
}

use rocket::response::NamedFile;

#[get("/fuchur.jpg")]
fn fuchur() -> std::io::Result<NamedFile> {
    NamedFile::open("fuchur.jpg")
}