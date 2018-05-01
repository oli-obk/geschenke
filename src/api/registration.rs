use pool::DbConn;

use diesel::pg::PgConnection;
use diesel::{self, QueryResult, RunQueryDsl};
use geschenke::NewUser;
use geschenke::UserCreationError;
use mail::Mail;
use rand;
use rand::distributions::{IndependentSample, Range};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket::State;
use ui::localization::Lang;
use ui::send_mail;

#[derive(Deserialize, FromForm)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[post("/register_form", data = "<user>")]
fn create_user_form(
    conn: DbConn,
    mailstrom: State<Mail>,
    user: Form<User>,
    lang: Lang,
) -> QueryResult<Flash<Redirect>> {
    match create_user(&*conn, mailstrom, &user.get(), lang) {
        Ok(()) => Ok(Flash::success(
            Redirect::to("/"),
            format!(
                "An email with login instructions has been sent to {}",
                user.get().email
            ),
        )),
        Err(err) => user_creation_error(err),
    }
}

pub fn gen_autologin() -> String {
    let mut autologin = String::new();
    let range = Range::new(b'a', b'z');
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let c = range.ind_sample(&mut rng);
        autologin.push(c as char)
    }
    autologin
}

pub fn try_create_user(
    conn: &PgConnection,
    user: &User,
) -> Result<(bool, String), UserCreationError> {
    let email_address = user.email.trim();
    if email_address.chars().any(|c| c.is_whitespace())
        || email_address.chars().filter(|&c| c == '@').count() != 1
    {
        return Err(UserCreationError::InvalidEmailAddress);
    }
    use geschenke::schema::users;

    let autologin = gen_autologin();

    let count = diesel::insert_into(users::table)
        .values(&NewUser {
            name: &user.name,
            email: email_address,
            autologin: &autologin,
        })
        .execute(conn)?;
    assert!(count <= 1, "inserting either inserts 1 row or 0");
    Ok((count == 1, autologin))
}

pub fn user_creation_error(
    err: UserCreationError
) -> QueryResult<Flash<Redirect>> {
    match err {
        UserCreationError::EmailAlreadyExists => Ok(Flash::error(
            Redirect::to("/"),
            "This email is already registered",
        )),
        UserCreationError::InvalidEmailAddress => Ok(Flash::error(
            Redirect::to("/"),
            "That's not an email address",
        )),
        UserCreationError::CouldNotSendMail => Ok(Flash::error(
            Redirect::to("/"),
            "Please contact an admin, emails could not be sent",
        )),
        UserCreationError::Diesel(diesel) => Err(diesel),
    }
}

fn create_user(
    conn: &PgConnection,
    mailstrom: State<Mail>,
    user: &User,
    lang: Lang,
) -> Result<(), UserCreationError> {
    let (new, autologin) = try_create_user(conn, user)?;
    if new {
        // added new entry
        send_mail(
            mailstrom,
            lang,
            &user.email,
            "Geschenke App Registration",
            "registration-mail",
            fluent_map!{
                "email_address" => user.email.clone(),
                "autologin" => autologin,
            },
        );
    } else {
        // just send a new email with a key, the user probably forgot they had an account
        ::api::account::recover_login(conn, &user.email, &autologin, mailstrom, lang)?;
    }

    Ok(())
}
