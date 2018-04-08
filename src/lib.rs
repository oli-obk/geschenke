pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate sha3;
extern crate rand;
extern crate mailstrom;
extern crate email_format;
#[macro_use] extern crate serde_derive;

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;
use rand::distributions::{IndependentSample, Range};

use self::models::NewUser;
pub use self::models::NewGeschenk;
pub use self::models::Geschenk;

pub type UserId = i32;
pub type GeschenkId = i32;
pub type AutologinKey = String;
pub type BorrowedAutologinKey = str;

pub enum Error {
    DieselError(DieselError),
}

// Returns `None` if the autologin key does not exist
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

// Returns `None` if the email does not exist,
// or the user has no password, or the password is wrong
pub fn login_with_password(
    conn: &PgConnection,
    email_address: &str,
    pw: &str,
) -> QueryResult<Option<UserId>> {
    use schema::users::dsl as users;
    use sha3::{Digest, Sha3_512};

    users::users
        .select((users::password, users::salt, users::id))
        .filter(users::email.eq(email_address))
        .get_result::<(Option<String>, Option<String>, UserId)>(conn)
        .optional()
        .map(|result| result.and_then(|(password_hash, salt, id)| {
            let password_hash = password_hash?;
            let salt = salt?;
            let mut hasher = Sha3_512::default();
            hasher.input(pw.as_bytes());
            hasher.input(salt.as_bytes());
            if hasher.result().as_slice() == password_hash.as_bytes() {
                Some(id)
            } else {
                None
            }
        }))
}

pub fn set_pw(conn: &PgConnection, user: UserId, pw: String) -> QueryResult<()> {
    unimplemented!()
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

    // send email

    diesel::insert_into(users::table)
        .values(&NewUser {
            name,
            email,
            autologin: &autologin,
        })
        .returning(users::id)
        .get_result::<UserId>(conn)
}

pub fn show_presents_for_user(conn: &PgConnection, viewer: UserId, recipient: UserId) -> QueryResult<Vec<Geschenk>> {
    use schema::{geschenke, friends};

    let query = geschenke::table
        .filter(geschenke::receiver.eq(recipient));

    if viewer == recipient {
        // show only the presents that the user created himself
        query.filter(geschenke::creator.eq(viewer))
            .load::<Geschenk>(&*conn)
    } else {
        let is_friend = friends::friend.eq(recipient);
        let viewer_friends = friends::id.eq(viewer).and(is_friend).and(geschenke::receiver.ne(viewer));

        geschenke::table
            .inner_join(friends::table.on(viewer_friends))
            .select((
                // TODO: find a better way to build this select
                geschenke::id,
                geschenke::short_description,
                geschenke::description,
                geschenke::creator,
                geschenke::receiver,
                geschenke::gifter,
                geschenke::obtained_date,
                geschenke::gifted_date,
            ))
            .load::<Geschenk>(&*conn)
    }
}

pub fn get_present(conn: &PgConnection, viewer: UserId, geschenk: GeschenkId) -> QueryResult<Geschenk> {
    use schema::{geschenke, friends};

    let check_id = geschenke::id.eq(geschenk);
    let created_by_viewer = geschenke::creator.eq(viewer);
    let is_friend = friends::friend.eq(geschenke::receiver);
    let viewer_friends = friends::id.eq(viewer).and(is_friend);

    geschenke::table
        .inner_join(friends::table.on(viewer_friends.or(created_by_viewer)))
        .filter(check_id)
        .select((
            // TODO: find a better way to build this select
            geschenke::id,
            geschenke::short_description,
            geschenke::description,
            geschenke::creator,
            geschenke::receiver,
            geschenke::gifter,
            geschenke::obtained_date,
            geschenke::gifted_date,
        ))
        .get_result::<Geschenk>(&*conn)
}

// password recovery
