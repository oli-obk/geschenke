pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::NewUser;

pub enum Error {
    DieselError(DieselError),
}

pub fn create_user<'a>(conn: &PgConnection, name: &'a str, email: &'a str) -> QueryResult<()> {
    use schema::users;

    let new_user = NewUser {
        name,
        email,
    };

    let n = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    assert_eq!(n, 1, "create_user only adds one entry");
    Ok(())
}
