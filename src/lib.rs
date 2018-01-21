pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate sha3;
extern crate rand;
#[macro_use] extern crate serde_derive;


use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;
use rand::distributions::{IndependentSample, Range};

use self::models::NewUser;
use self::models::NewGeschenk;
use self::models::Geschenk;

pub type UserId = i32;
pub type GeschenkId = i32;
pub type AutologinKey = String;
pub type BorrowedAutologinKey = str;

pub enum Error {
    DieselError(DieselError),
}

pub fn login_with_key(
    conn: &PgConnection,
    key: &BorrowedAutologinKey,
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

pub fn set_pw(conn: &PgConnection, user: UserId, pw: String) -> QueryResult<()> {
    unimplemented!()
}

pub fn create_user(conn: &PgConnection, name: &str, email: &str) -> QueryResult<(UserId, AutologinKey)> {
    use schema::users;

    let mut autologin = String::new();
    let range = Range::new(b'a', b'z');
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let c = range.ind_sample(&mut rng);
        autologin.push(c as char)
    }

    diesel::insert_into(users::table)
        .values(&NewUser {
            name,
            email,
            autologin: &autologin,
        })
        .returning(users::id)
        .get_result::<UserId>(conn)
        .map(|id| (id, autologin))
}

pub fn add_present(conn: &PgConnection, creator: UserId, recipient: UserId, short_description: &str, description: &str) -> QueryResult<GeschenkId> {
    use schema::geschenke;

    let new_geschenk = NewGeschenk {
        creator: Some(creator),
        receiver: recipient,
        short_description,
        description,
    };

    diesel::insert_into(geschenke::table)
        .values(&new_geschenk)
        .returning(geschenke::id)
        .get_result::<GeschenkId>(conn)
}

pub fn show_presents_for_user(conn: &PgConnection, viewer: UserId, recipient: UserId) -> QueryResult<Vec<Geschenk>> {
    use schema::geschenke;

    let query = geschenke::table
        .filter(geschenke::receiver.eq(recipient));

    if viewer == recipient {
        // show only the presents that the user created himself
        query.filter(geschenke::creator.eq(viewer))
            .load::<Geschenk>(&*conn)
    } else {
        query.load::<Geschenk>(&*conn)
    }
}

// password recovery
