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
        enum Info {
            SelfHugging,
            Already,
            Ok,
        }
        let try = |a, b| {
            let result = insert_into(friends::table)
                .values(&NewFriend {
                    friend: a,
                    id: b,
                })
                .execute(&*conn);
            use diesel::result::{Error, DatabaseErrorKind};
            match result {
                Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Ok(Info::Already),
                Err(Error::DatabaseError(DatabaseErrorKind::__Unknown, ref info)) if info.constraint_name() == Some("no_self_hugging") => Ok(Info::SelfHugging),
                Ok(_) => Ok(Info::Ok),
                Err(other) => Err(other),
            }
        };
        // we already have this friend
        match try(friend, user.0)? {
            Info::Ok => {},
            Info::Already => return Ok(Flash::error(Redirect::to("/"), "You are already friends")),
            Info::SelfHugging => return Ok(Flash::error(Redirect::to("/"), "You cannot befriend yourself")),
        }
        // if we didn't have the friend, we do have them now
        // try adding the reverse frienship, but ignore if already exists
        try(user.0, friend)?;
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
