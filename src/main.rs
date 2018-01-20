#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate geschenke;
extern crate diesel;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;

mod api;
mod pool;

fn main() {
    rocket::ignite()
        .mount("/", routes![api::hello])
        .mount("/geschenke", routes![api::debugging::get_geschenke])
        .mount("/users", routes![api::debugging::get_users])
        .mount("/registration", routes![
            api::registration::create_user,
            api::registration::create_user_form,
        ])
        .manage(pool::establish_connection())
        .launch();
}
