use pool::DbConn;
use rocket::response::{Flash, Redirect};
use rocket::request::Form;
use geschenke::schema::{users, friends};
use diesel::prelude::*;
use diesel::{delete, insert_into};
use geschenke::models::NewFriend;
use super::UserId;

#[derive(Deserialize, FromForm)]
pub struct AddUser {
    email: String,
}

#[derive(Deserialize, FromForm)]
pub struct RemoveUser {
    id: ::geschenke::UserId,
}

#[post("/friend/add", data = "<new_friend>")]
pub fn add_friend(conn: DbConn, user: UserId, new_friend: Form<AddUser>) -> QueryResult<Flash<Redirect>> {
    let friend_info = users::table
        .filter(users::email.eq(&new_friend.get().email))
        .select(users::id)
        .get_result::<::geschenke::UserId>(&*conn)
        .optional()?;
    if let Some(friend) = friend_info {
        insert_into(friends::table)
            .values(&NewFriend {
                friend,
                id: user.0,
            })
            .execute(&*conn)?;
        // also add the other direction of friendship
        insert_into(friends::table)
            .values(&NewFriend {
                friend: user.0,
                id: friend,
            })
            .execute(&*conn)?;
        Ok(Flash::error(Redirect::to("/"), "Added new friend"))
    } else {
        Ok(Flash::error(Redirect::to("/"), "Could not add unregistered friend"))
    }
}

#[post("/friend/remove", data = "<delete_friend>")]
pub fn remove_friend(conn: DbConn, user: UserId, delete_friend: Form<RemoveUser>) -> QueryResult<Flash<Redirect>> {
    let id = friends::id.eq(user.0);
    let friend_id = friends::friend.eq(delete_friend.get().id);
    let query = friends::table.filter(id.and(friend_id));
    if delete(query).execute(&*conn).is_ok() {
        Ok(Flash::success(Redirect::to("/"), "Deleted friend"))
    } else {
        Ok(Flash::error(Redirect::to("/"), "Could not delete friend"))
    }
}
