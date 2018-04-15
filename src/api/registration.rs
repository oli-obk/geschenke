use pool::DbConn;

use geschenke::UserCreationError;
use diesel::{QueryResult, self, RunQueryDsl};
use diesel::pg::PgConnection;
use rocket::request::Form;
use rocket::response::{Redirect, Flash};
use rocket::State;
use mail::Mail;
use geschenke::{UserId, NewUser};
use rand::distributions::{IndependentSample, Range};
use rand;
use chrono::prelude::*;

#[derive(Deserialize, FromForm)]
struct User {
    name: String,
    email: String,
}

#[post("/register_form", data = "<user>")]
fn create_user_form(conn: DbConn, mailstrom: State<Mail>, user: Form<User>) -> QueryResult<Flash<Redirect>> {
    let email = &user.get().email;
    match create_user(&*conn, mailstrom, &user.get().name, email) {
        Ok(_) => Ok(Flash::success(Redirect::to("/"), format!("An email with login instructions has been sent to {}", email))),
        Err(UserCreationError::EmailAlreadyExists) => Ok(Flash::error(Redirect::to("/"), "This email is already registered")),
        Err(UserCreationError::InvalidEmailAddress) => Ok(Flash::error(Redirect::to("/"), "That's not an email address")),
        Err(UserCreationError::CouldNotSendMail) => Ok(Flash::error(Redirect::to("/"), "Please contact an admin, emails could not be sent")),
        Err(UserCreationError::Diesel(diesel)) => Err(diesel),
    }
}

use email_format::Email;

fn create_user(conn: &PgConnection, mailstrom: State<Mail>, name: &str, email_address: &str) -> Result<UserId, UserCreationError> {
    let email_address = email_address.trim();
    if email_address.chars().any(|c| c.is_whitespace()) || email_address.chars().filter(|&c| c == '@').count() != 1 {
        return Err(UserCreationError::InvalidEmailAddress);
    }
    use geschenke::schema::users;

    let mut autologin = String::new();
    let range = Range::new(b'a', b'z');
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let c = range.ind_sample(&mut rng);
        autologin.push(c as char)
    }

    let now: DateTime<Utc> = Utc::now();
    let mut email = Email::new(
        "geschenke@oli-obk.de",  // "From:"
        &now, // "Date:"
    ).unwrap();

    email.set_sender("geschenke@oli-obk.de").unwrap();
    email.set_to(email_address).unwrap();
    email.set_subject("Geschenke App Registration").unwrap();
    let body = format!(
        "Someone (probably you) has created an account for this email address at https://geschenke.oli-obk.de .\r\n\
        \r\n\
        Click the following link to login:\r\n\
        https://geschenke.oli-obk.de/account/login_form_key?key={} \r\n\
        \r\n\
        Your friendly neighborhood Geschenke-Bot\r\n\
        \r\n\
        \r\n\
        If it was not you, visit\r\n\
        https://geschenke.oli-obk.de/nuke/{}/{}.\r\n\
        to remove your email address from our database\r\n",
        email_address,
        autologin,
        autologin,
    );
    println!("{:?}", body);
    email.set_body(&*body).unwrap();

    mailstrom.lock().unwrap().send_email(email).unwrap();

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
