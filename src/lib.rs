pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
extern crate sha3;

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

pub fn login_with_key(
    conn: &PgConnection,
    key: &str,
) -> QueryResult<Option<i32>> {
    use schema::users::dsl as users;
    users::users
        .select(users::id)
        .filter(users::autologin.eq(key))
        .get_result::<i32>(conn)
        .optional()
}

pub fn login_with_password(
    conn: &PgConnection,
    email_address: &str,
    pw: &str,
) -> QueryResult<bool> {
    use schema::users::dsl::*;
    use sha3::{Digest, Sha3_512};

    let (password_hash, db_salt) = users
        .select((password, salt))
        .filter(email.eq(email_address))
        .get_result::<(Option<String>, Option<String>)>(conn)?;
    let (password_hash, db_salt) = match (password_hash, db_salt) {
        (Some(pw), Some(s)) => (pw, s),
        _ => return Ok(false),
    };
    let mut hasher = Sha3_512::default();
    hasher.input(pw.as_bytes());
    hasher.input(db_salt.as_bytes());
    Ok(hasher.result().as_slice() == password_hash.as_bytes())
}

pub fn create_user(conn: &PgConnection, name: &str, email: &str) -> QueryResult<()> {
    use schema::users;

    let new_user = NewUser {
        name,
        email,
        autologin: "foomp",
    };

    let n = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    assert_eq!(n, 1, "create_user only adds one entry");
    Ok(())
}
