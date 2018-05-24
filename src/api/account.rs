use diesel;
use diesel::prelude::*;
use geschenke::schema::users;
use geschenke::{login_with_key, login_with_password};
use mail::Mail;
use pool::DbConn;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket::State;
use ui::localization::Lang;
use ui::send_mail;

/// Remove the `user_id` cookie.
#[get("/logout")]
fn logout(mut cookies: Cookies, lang: Lang) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/"), lang.format("logout-successful", None))
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
struct KeyLogin {
    key: String,
    forward: Option<String>,
}

#[get("/login_form_key?<login>")]
fn login_key(conn: DbConn, mut cookies: Cookies, login: KeyLogin, lang: Lang) -> QueryResult<Flash<Redirect>> {
    if let Some(id) = login_with_key(&*conn, &login.key)? {
        cookies.add_private(Cookie::new("user_id", id.to_string()));
        let target = login.forward.as_ref().map_or("/", |f| f);
        Ok(Flash::success(Redirect::to(target), lang.format("login-successful", None)))
    } else {
        Ok(Flash::error(Redirect::to("/"), lang.format("wrong-key", None)))
    }
}

#[post("/login_form", data = "<login>")]
fn login(conn: DbConn, mut cookies: Cookies, login: Form<Login>, lang: Lang) -> QueryResult<Flash<Redirect>> {
    if let Some(id) = login_with_password(&*conn, &login.get().email, &login.get().password)? {
        cookies.add_private(Cookie::new("user_id", id.to_string()));
        Ok(Flash::success(Redirect::to("/"), lang.format("login-successful", None)))
    } else {
        Ok(Flash::error(
            Redirect::to("/"),
            lang.format("wrong-mail-or-password", None),
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
    recover_login(&*conn, email, &new_autologin, mailstrom, lang.clone())?;

    // we don't leak whether that user has an account
    Ok(Flash::success(
        Redirect::to("/"),
        lang.format("new-key-sent", None),
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
        send_mail(
            &mailstrom,
            lang.clone(),
            email_address,
            &lang.format("login-subject", None),
            "login-mail",
            fluent_map!{
                "autologin" => new_autologin,
            },
        );
    }
    Ok(())
}
