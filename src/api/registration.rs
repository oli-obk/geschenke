use pool::DbConn;

use geschenke::{self, UserCreationError};
use diesel::QueryResult;
use rocket::request::Form;
use rocket::response::{Redirect, Flash};

#[derive(Deserialize, FromForm)]
struct User {
    name: String,
    email: String,
}

#[post("/register_form", data = "<user>")]
fn create_user_form(conn: DbConn, user: Form<User>) -> QueryResult<Flash<Redirect>> {
    match geschenke::create_user(&*conn, &user.get().name, &user.get().email) {
        Ok(_) => Ok(Flash::success(Redirect::to("/"), "An email with your password has been sent to you")),
        Err(UserCreationError::EmailAlreadyExists) => Ok(Flash::error(Redirect::to("/"), "This email is already registered")),
        Err(UserCreationError::InvalidEmailAddress) => Ok(Flash::error(Redirect::to("/"), "That's not an email address")),
        Err(UserCreationError::CouldNotSendMail) => Ok(Flash::error(Redirect::to("/"), "Please contact an admin, emails could not be sent")),
        Err(UserCreationError::Diesel(diesel)) => Err(diesel),
    }
}
