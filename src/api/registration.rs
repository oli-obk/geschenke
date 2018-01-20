use pool::DbConn;
use rocket_contrib::Json;

use geschenke::{self, AutologinKey};
use diesel::QueryResult;
use rocket::request::Form;

#[derive(Deserialize, FromForm)]
struct User {
    name: String,
    email: String,
}

#[post("/register", format = "application/json", data = "<user>")]
fn create_user(conn: DbConn, user: Json<User>) -> QueryResult<Json<AutologinKey>> {
    let (_, key) = geschenke::create_user(&*conn, &user.name, &user.email)?;
    Ok(Json(key))
}

#[post("/register_form", data = "<user>")]
fn create_user_form(conn: DbConn, user: Form<User>) -> QueryResult<Json<AutologinKey>> {
    let (_, key) = geschenke::create_user(&*conn, &user.get().name, &user.get().email)?;
    Ok(Json(key))
}
