use rocket_contrib::Json;
use rocket::response::Content;
use rocket::http::ContentType;

use geschenke::models::{Geschenk, User};
use geschenke::schema::geschenke;
use geschenke::schema::users;
use diesel::{QueryResult, RunQueryDsl};
use pool::DbConn;

#[get("/")]
fn hello() -> Content<&'static str> {
    Content(ContentType::HTML, include_str!("index.html"))
}

#[get("/")]
fn get_users(conn: DbConn) -> QueryResult<Json<Vec<User>>> {
    users::table
        .load::<User>(&*conn)
        .map(Json)
}

#[get("/")]
fn get_geschenke(conn: DbConn) -> QueryResult<Json<Vec<Geschenk>>> {
    geschenke::table
        .load::<Geschenk>(&*conn)
        .map(Json)
}
