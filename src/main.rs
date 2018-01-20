#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate geschenke;
extern crate diesel;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;

extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;

mod api;
mod pool;

fn main() {
    rocket::ignite()
        .mount("/", routes![api::hello])
        .mount("/geschenke", routes![api::get_geschenke])
        .mount("/users", routes![api::get_users])
        .manage(pool::establish_connection())
        .launch();
}
