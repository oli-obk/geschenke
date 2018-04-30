use chrono::prelude::*;
use diesel;
use diesel::prelude::*;
use email_format::Email;
use geschenke::schema::users;
use geschenke::{login_with_key, login_with_password};
use mail::Mail;
use pool::DbConn;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket::State;
use ui::localization::Lang;

/// Remove the `user_id` cookie.
#[get("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

#[derive(Deserialize, FromForm)]
struct Login {
    email: String,
    password: String,
}

#[derive(Deserialize, FromForm)]
struct Recover {
    email: String,
}

#[derive(FromForm)]
struct Key {
    key: String,
}

#[get("/login_form_key?<key>")]
fn login_key(conn: DbConn, mut cookies: Cookies, key: Key) -> QueryResult<Flash<Redirect>> {
    if let Some(id) = login_with_key(&*conn, &key.key)? {
        cookies.add_private(Cookie::new("user_id", id.to_string()));
        Ok(Flash::success(Redirect::to("/"), "Successfully logged in."))
    } else {
        Ok(Flash::error(Redirect::to("/"), "Wrong or old login key"))
    }
}

#[post("/login_form", data = "<login>")]
fn login(conn: DbConn, mut cookies: Cookies, login: Form<Login>) -> QueryResult<Flash<Redirect>> {
    if let Some(id) = login_with_password(&*conn, &login.get().email, &login.get().password)? {
        cookies.add_private(Cookie::new("user_id", id.to_string()));
        Ok(Flash::success(Redirect::to("/"), "Successfully logged in."))
    } else {
        Ok(Flash::error(
            Redirect::to("/"),
            "Unknown email address or wrong password",
        ))
    }
}

#[post("/recover", data = "<recover>")]
fn recover(
    conn: DbConn,
    recover: Form<Recover>,
    mailstrom: State<Mail>,
    lang: Lang,
) -> QueryResult<Flash<Redirect>> {
    let email = &recover.get().email;
    let new_autologin = ::api::registration::gen_autologin();
    recover_login(&*conn, email, &new_autologin, mailstrom, lang)?;

    // we don't leak whether that user has an account
    Ok(Flash::success(
        Redirect::to("/"),
        "Email with new login key sent",
    ))
}

pub fn recover_login(
    conn: &PgConnection,
    email_address: &str,
    new_autologin: &str,
    mailstrom: State<Mail>,
    lang: Lang,
) -> QueryResult<()> {
    let updated = diesel::update(users::table.filter(users::email.eq(email_address)))
        .set(users::autologin.eq(new_autologin))
        .execute(conn)?;
    assert!(updated <= 1);
    if updated == 1 {
        let now: DateTime<Utc> = Utc::now();
        let mut email = Email::new(
            "geschenke@oli-obk.de", // "From:"
            &now,                   // "Date:"
        ).unwrap();

        email.set_sender("geschenke@oli-obk.de").unwrap();
        email.set_to(email_address).unwrap();
        email.set_subject("Geschenke App Login").unwrap();
        let body = lang.format(
            "login-mail",
            fluent_map!{
                "autologin" => new_autologin,
            },
        );
        email.set_body(&*body).unwrap();

        mailstrom.lock().unwrap().send_email(email).unwrap();
    }
    Ok(())
}
