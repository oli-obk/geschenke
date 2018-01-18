#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate geschenke;
extern crate diesel;
extern crate rocket;

use self::geschenke::*;
use self::geschenke::models::*;
use self::diesel::prelude::*;

pub mod api;

fn main() {
    use geschenke::schema::users::dsl::*;
    use geschenke::schema::geschenke::dsl::*;

    let connection = establish_connection();
    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{} <{}>: {}", user.name, user.email, user.autologin);
    }

    let results = geschenke
        .load::<Geschenk>(&connection)
        .expect("Error loading geschenke");

    println!("Displaying {} geschenke", results.len());
    for geschenk in results {
        println!("{}", geschenk.short_description);
    }

    //create_user(&connection, "a", "b").unwrap();
    println!("{:?}", login_with_key(&connection, "bar"));

    rocket::ignite().mount("/", routes![api::hello]).launch();
}
