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
            api::registration::create_user,
            api::registration::create_user_form,
        ])
        .mount("/account", routes![
            api::account::logout,
            api::account::login,
            api::account::login_key,
        ])
        .manage(pool::establish_connection())
        .launch();
}
