use rocket::response::Content;
use rocket::http::ContentType;
use horrorshow::prelude::*;
use horrorshow::helper::doctype;
use pool::DbConn;
use diesel::QueryResult;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, Request, FromRequest, FlashMessage};

pub mod debugging;
pub mod registration;
pub mod account;
pub mod logged_in;
pub mod geschenk;
pub mod user;

/// Automatically obtains a user id from cookies
pub struct UserId(::geschenke::UserId);

impl<'a, 'r> FromRequest<'a, 'r> for UserId {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserId, ()> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(UserId)
            .or_forward(())
    }
}

#[get("/")]
fn hello(conn: DbConn, user: Option<UserId>, flash: Option<FlashMessage>) -> QueryResult<Content<String>> {
    let page = if let Some(user) = user {
        logged_in::hello_user(conn, user, flash)?
    } else {
        hello_generic()
    };
    Ok(Content(ContentType::HTML, page))
}

fn hello_generic() -> String {
    html! (
        : doctype::HTML;
        html {
            head {
                title : "Hello World!";
            }
            body {
                h1 { : "Debugging" }
                a(href="debugging/geschenke") { : "Database dump of presents" } br;
                a(href="debugging/users") { : "Database dump of users"} br;
                a(href="debugging/user_info") { : "info about current user"} br;
                h1 { : "Login" }
                form(action="account/login_form", method="post") {
                    :"Email:"; input(name="email"); br;
                    :"Password:"; input(type="password", name="password"); br;
                    button { : "Login" }
                }
                h1 { : "Login with key" }
                form(action="account/login_form_key", method="get") {
                    :"Key:"; input(name="key"); br;
                    button { : "Login" }
                }
                h1 { : "Register" }
                form(action="registration/register_form", method="post") {
                    :"Name:";  input(name="name" ); br;
                    :"Email:"; input(name="email"); br;
                    button { : "Create User" }
                }
            }
        }
    ).into_string().unwrap()
}
