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
use chrono::prelude::*;

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

#[derive(Debug, PartialEq)]
pub enum UserCreationError {
    /// There's already an entry with that email in the database
    EmailAlreadyExists,
    /// Multiple `@` or contains spaces
    InvalidEmailAddress,
    /// Server failure, probably because of blacklisted ips
    CouldNotSendMail,
    /// Generic diesel error
    Diesel(::diesel::result::Error),
}

impl From<::diesel::result::Error> for UserCreationError {
    fn from(other: ::diesel::result::Error) -> Self {
        UserCreationError::Diesel(other)
    }
}

pub fn create_user(conn: &PgConnection, name: &str, email_address: &str) -> Result<UserId, UserCreationError> {
    let email_address = email_address.trim();
    if email_address.chars().any(|c| c.is_whitespace()) || email_address.chars().filter(|&c| c == '@').count() != 1 {
        return Err(UserCreationError::InvalidEmailAddress);
    }
    use schema::users;

    let mut autologin = String::new();
    let range = Range::new(b'a', b'z');
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let c = range.ind_sample(&mut rng);
        autologin.push(c as char)
    }

    use email_format::Email;
    use mailstrom::{Mailstrom, Config, MemoryStorage};
    let now: DateTime<Utc> = Utc::now();
    let mut email = Email::new(
        "geschenke@oli-obk.de",  // "From:"
        &now, // "Date:"
    ).unwrap();

    email.set_bcc("geschenke@oli-obk.de").unwrap();
    email.set_sender("geschenke@oli-obk.de").unwrap();
    email.set_to(email_address).unwrap();
    email.set_subject("Geschenke App Registration").unwrap();
    let body = format!(
        "Someone (probably you) has created an account for this email address at https://geschenke.oli-obk.de .\r\n\
        If it was not you, visit https://geschenke.oli-obk.de/nuke/{}/{}.\r\n\
        \r\n\
        Click the following link to login: https://geschnek.oli-obk.de/login/{} \r\n\
        \r\n\
        Your friendly neighborhood Geschenke-Bot",
        email_address,
        autologin,
        autologin,
    );
    println!("{:?}", body);
    email.set_body(&*body).unwrap();

    let mut mailstrom = Mailstrom::new(
        Config {
            helo_name: "geschenke.oli-obk.de".to_owned(),
            smtp_timeout_secs: 30,
            ..Default::default()
        },
        MemoryStorage::new());

    // We must explicitly tell mailstrom to start actually sending emails.  If we
    // were only interested in reading the status of previously sent emails, we
    // would not send this command.
    mailstrom.start().unwrap();

    let message_id = mailstrom.send_email(email).unwrap();

    // Later on, after the worker thread has had time to process the request,
    // you can check the status:

    loop {
        let status = mailstrom.query_status(&*message_id).unwrap();
        println!("{:?}", status);
        if status.completed() {
            if !status.succeeded() {
                return Err(UserCreationError::CouldNotSendMail);
            }
            break;
        }
    }

    let id = diesel::insert_into(users::table)
        .values(&NewUser {
            name,
            email: email_address,
            autologin: &autologin,
        })
        .returning(users::id)
        .get_result::<UserId>(conn)?;
    Ok(id)
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
