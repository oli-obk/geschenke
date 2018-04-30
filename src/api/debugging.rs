use rocket_contrib::Json;

use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use geschenke::models::{Present, User};
use geschenke::schema::presents;
use geschenke::schema::users;
use geschenke::UserId;
use pool::DbConn;
use rocket::http::Cookies;

#[get("/users")]
fn get_users(conn: DbConn) -> QueryResult<Json<Vec<User>>> {
    users::table.load::<User>(&*conn).map(Json)
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

#[get("/presents")]
fn get_presents(conn: DbConn) -> QueryResult<Json<Vec<Present>>> {
    presents::table.load::<Present>(&*conn).map(Json)
}
