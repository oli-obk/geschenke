extern crate geschenke;
extern crate diesel;

use self::geschenke::*;
use self::geschenke::models::*;
use self::diesel::prelude::*;

fn main() {
    use geschenke::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.name);
        println!("----------\n");
        println!("{}", user.email);
    }
}
