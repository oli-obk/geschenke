use pool::DbConn;
use rocket::http::{Cookies, Cookie};
use rocket::response::{Flash, Redirect};
use rocket::request::Form;
use geschenke::{login_with_password, login_with_key};
use diesel::prelude::*;

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

#[derive(FromForm)]
struct Key {
    key: String
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
        Ok(Flash::error(Redirect::to("/"), "Unknown email address or wrong password"))
    }
}
