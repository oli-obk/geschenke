#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate geschenke;
extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;
#[macro_use]
extern crate horrorshow;

use rocket::Request;

mod api;
mod pool;

fn main() {
    rocket::ignite()
        .mount("/", routes![api::hello])
        .mount("/debugging", routes![
            api::debugging::get_geschenke,
            api::debugging::get_users,
            api::debugging::user_info,
        ])
        .mount("/registration", routes![
            api::registration::create_user_form,
        ])
        .mount("/account", routes![
            api::account::logout,
            api::account::login,
            api::account::login_key,
        ])
        .mount("/geschenk", routes![
            api::geschenk::edit,
            api::geschenk::view,
            api::geschenk::add,
        ])
        .mount("/user", routes![
            api::user::add_friend,
            api::user::remove_friend,
            api::user::view,
        ])
        .catch(errors![
            not_found,
            bad_parse,
        ])
        .manage(pool::establish_connection())
        .launch();
}

#[error(404)]
fn not_found(req: &Request) -> String {
    format!("{:#?}", req)
}

#[error(422)]
fn bad_parse(req: &Request) -> String {
    format!("{:#?}", req)
}
