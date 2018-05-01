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
struct User {
    name: String,
    email: String,
}

#[post("/register_form", data = "<user>")]
fn create_user_form(
    conn: DbConn,
    mailstrom: State<Mail>,
    user: Form<User>,
    lang: Lang,
) -> QueryResult<Flash<Redirect>> {
    let email = &user.get().email;
    match create_user(&*conn, mailstrom, &user.get().name, email, lang) {
        Ok(()) => Ok(Flash::success(
            Redirect::to("/"),
            format!(
                "An email with login instructions has been sent to {}",
                email
            ),
        )),
        Err(UserCreationError::EmailAlreadyExists) => Ok(Flash::error(
            Redirect::to("/"),
            "This email is already registered",
        )),
        Err(UserCreationError::InvalidEmailAddress) => Ok(Flash::error(
            Redirect::to("/"),
            "That's not an email address",
        )),
        Err(UserCreationError::CouldNotSendMail) => Ok(Flash::error(
            Redirect::to("/"),
            "Please contact an admin, emails could not be sent",
        )),
        Err(UserCreationError::Diesel(diesel)) => Err(diesel),
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

fn create_user(
    conn: &PgConnection,
    mailstrom: State<Mail>,
    name: &str,
    email_address: &str,
    lang: Lang,
) -> Result<(), UserCreationError> {
    let email_address = email_address.trim();
    if email_address.chars().any(|c| c.is_whitespace())
        || email_address.chars().filter(|&c| c == '@').count() != 1
    {
        return Err(UserCreationError::InvalidEmailAddress);
    }
    use geschenke::schema::users;

    let autologin = gen_autologin();

    let count = diesel::insert_into(users::table)
        .values(&NewUser {
            name,
            email: email_address,
            autologin: &autologin,
        })
        .execute(conn)?;

    if count == 1 {
        // added new entry
        send_mail(
            mailstrom,
            lang,
            email_address,
            "Geschenke App Registration",
            "registration-mail",
            fluent_map!{
                "email_address" => email_address,
                "autologin" => autologin,
            },
        );
    } else if count == 0 {
        // just send a new email with a key, the user probably forgot they had an account
        ::api::account::recover_login(conn, email_address, &autologin, mailstrom, lang)?;
    } else {
        panic!("inserting either inserts 1 row or 0");
    }

    Ok(())
}
