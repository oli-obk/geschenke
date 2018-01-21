use rocket_contrib::Json;

use geschenke::models::{Geschenk, User};
use geschenke::schema::geschenke;
use geschenke::schema::users;
use geschenke::UserId;
use rocket::http::Cookies;
use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods};
use pool::DbConn;

#[get("/users")]
fn get_users(conn: DbConn) -> QueryResult<Json<Vec<User>>> {
    users::table
        .load::<User>(&*conn)
        .map(Json)
}

#[get("/user_info")]
fn user_info(conn: DbConn, mut cookies: Cookies) -> QueryResult<Json<Option<User>>> {
    if let Some(id) = cookies.get_private("user_id") {
        let id: UserId = id.value().parse().unwrap();
        users::table
            .filter(users::id.eq(id))
            .get_result::<User>(&*conn)
            .map(Some)
            .map(Json)
    } else {
        Ok(Json(None))
    }
}

#[get("/geschenke")]
fn get_geschenke(conn: DbConn) -> QueryResult<Json<Vec<Geschenk>>> {
    geschenke::table
        .load::<Geschenk>(&*conn)
        .map(Json)
}
