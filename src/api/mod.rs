use diesel::QueryResult;
use horrorshow::helper::doctype;
use horrorshow::prelude::*;
use pool::DbConn;
use rocket::http::ContentType;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::Content;
use ui::localization::Lang;

pub mod account;
pub mod debugging;
pub mod logged_in;
pub mod present;
pub mod registration;
pub mod user;

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
/// Automatically obtains a user id from cookies
pub struct UserId(::geschenke::UserId);

impl<'a, 'r> FromRequest<'a, 'r> for UserId {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserId, ()> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(UserId)
            .or_forward(())
    }
}

#[get("/")]
fn hello(
    conn: DbConn,
    lang: Lang,
    user: Option<UserId>,
    flash: Option<FlashMessage>,
) -> QueryResult<Content<String>> {
    if let Some(user) = user {
        logged_in::hello_user(conn, user, lang, flash)
    } else {
        Ok(Content(ContentType::HTML, hello_generic(flash, lang)))
    }
}

fn hello_generic(flash: Option<FlashMessage>, lang: Lang) -> String {
    html! (
        : doctype::HTML;
        html {
            head {
                title : lang.format("app-name", None);
            }
            body {
                @if let Some(flash) = flash {
                    span (style = flash.name()) {: flash.msg() }
                    br;
                }
                @if option_env!("ROCKET_ENV").unwrap_or("development") == "development" {
                    h1 { : "Debugging" }
                    a(href="debugging/presents") { : "Database dump of presents" } br;
                    a(href="debugging/users") { : "Database dump of users"} br;
                    a(href="debugging/user_info") { : "info about current user"} br;
                }
                @if option_env!("ROCKET_ENV") != Some("production") {
                    h1 { : lang.format("login", None) }
                    form(action="account/login_form", method="post") {
                        :lang.format("mail", None); : ":"; input(name="email"); br;
                        :lang.format("password", None); : ":"; input(type="password", name="password"); br;
                        button { : lang.format("login", None) }
                    }
                    h1 { : lang.format("login-with-key", None) }
                    form(action="account/login_form_key", method="get") {
                        :lang.format("key", None); : ":"; input(name="key"); br;
                        button { : lang.format("login", None) }
                    }
                }
                : lang.format("info-login", None); br;
                h1 { : lang.format("register", None) }
                form(action="registration/register_form", method="post") {
                    :lang.format("name", None); : ":";  input(name="name" ); br;
                    :lang.format("mail", None); : ":"; input(name="email"); br;
                    button { : lang.format("register", None) }
                }
                h1 { : lang.format("forgotten-login", None) }
                : lang.format("info-enter-correct-email", None);
                form(action="account/recover", method="post") {
                    :lang.format("mail", None); : ":"; input(name="email"); br;
                    button { : lang.format("info-resend-login-mail", None) }
                }
            }
        }
    ).into_string()
        .unwrap()
}
