pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
extern crate sha3;
extern crate rand;


use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use rand::distributions::{IndependentSample, Range};

use self::models::NewUser;

pub type UserId = i32;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub enum Error {
    DieselError(DieselError),
}

pub fn login_with_key(
    conn: &PgConnection,
    key: &str,
) -> QueryResult<Option<UserId>> {
    use schema::users::dsl as users;
    users::users
        .select(users::id)
        .filter(users::autologin.eq(key))
        .get_result::<UserId>(conn)
        .optional()
}

pub fn login_with_password(
    conn: &PgConnection,
    email_address: &str,
    pw: &str,
) -> QueryResult<Option<UserId>> {
    use schema::users::dsl as users;
    use sha3::{Digest, Sha3_512};

    let (password_hash, db_salt, id) = users::users
        .select((users::password, users::salt, users::id))
        .filter(users::email.eq(email_address))
        .get_result::<(Option<String>, Option<String>, UserId)>(conn)?;
    let (password_hash, db_salt) = match (password_hash, db_salt) {
        (Some(pw), Some(s)) => (pw, s),
        _ => return Ok(None),
    };
    let mut hasher = Sha3_512::default();
    hasher.input(pw.as_bytes());
    hasher.input(db_salt.as_bytes());
    if hasher.result().as_slice() == password_hash.as_bytes() {
        Ok(Some(id))
    } else {
        Ok(None)
    }
}

pub fn create_user(conn: &PgConnection, name: &str, email: &str) -> QueryResult<UserId> {
    use schema::users;

    let mut autologin = String::new();
    let range = Range::new(b'a', b'z');
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let c = range.ind_sample(&mut rng);
        autologin.push(c as char)
    }
    let new_user = NewUser {
        name,
        email,
        autologin: &autologin,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(users::id)
        .get_result::<UserId>(conn)

    Ok(user_id)
}
