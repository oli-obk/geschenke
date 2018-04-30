use super::UserId;
use diesel::prelude::*;
use diesel::{delete, insert_into};
use geschenke::models::NewFriend;
use geschenke::schema::{friends, users};
use geschenke::show_presents_for_user;
use horrorshow::RenderOnce;
use pool::DbConn;
use rocket::request::FlashMessage;
use rocket::request::Form;
use rocket::response::Content;
use rocket::response::{Flash, Redirect};
use ui;

#[derive(Deserialize, FromForm)]
pub struct AddUser {
    email: String,
}

#[derive(Deserialize, FromForm)]
pub struct RemoveUser {
    id: ::geschenke::UserId,
}

#[post("/friend/add", data = "<new_friend>")]
pub fn add_friend(
    conn: DbConn,
    user: UserId,
    new_friend: Form<AddUser>,
) -> QueryResult<Flash<Redirect>> {
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
                .values(&NewFriend { friend: a, id: b })
                .execute(&*conn);
            use diesel::result::{DatabaseErrorKind, Error};
            match result {
                Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                    Ok(Info::Already)
                }
                Err(Error::DatabaseError(DatabaseErrorKind::__Unknown, ref info))
                    if info.constraint_name() == Some("no_self_hugging") =>
                {
                    Ok(Info::SelfHugging)
                }
                Ok(_) => Ok(Info::Ok),
                Err(other) => Err(other),
            }
        };
        // we already have this friend
        match try(friend, user.0)? {
            Info::Ok => {}
            Info::Already => return Ok(Flash::error(Redirect::to("/"), "You are already friends")),
            Info::SelfHugging => {
                return Ok(Flash::error(
                    Redirect::to("/"),
                    "You cannot befriend yourself",
                ))
            }
        }
        // if we didn't have the friend, we do have them now
        // try adding the reverse friendship, but ignore if already exists
        try(user.0, friend)?;
        Ok(Flash::error(Redirect::to("/"), "Added new friend"))
    } else {
        Ok(Flash::error(
            Redirect::to("/"),
            "Could not add unregistered friend",
        ))
    }
}

#[post("/friend/remove", data = "<delete_friend>")]
pub fn remove_friend(
    conn: DbConn,
    user: UserId,
    delete_friend: Form<RemoveUser>,
) -> QueryResult<Flash<Redirect>> {
    let id = friends::id.eq(user.0);
    let friend_id = friends::friend.eq(delete_friend.get().id);
    let query = friends::table.filter(id.and(friend_id));
    if delete(query).execute(&*conn).is_ok() {
        Ok(Flash::success(Redirect::to("/"), "Deleted friend"))
    } else {
        Ok(Flash::error(Redirect::to("/"), "Could not delete friend"))
    }
}

pub fn print_wishlist(
    conn: DbConn,
    me: UserId,
    user: ::geschenke::UserId,
) -> QueryResult<(impl RenderOnce, String)> {
    let you = me.0 == user;
    let title = if you {
        "Your wishlist".to_owned()
    } else {
        let name = users::table
            .filter(users::id.eq(user))
            .inner_join(friends::table.on(friends::id.eq(me.0).and(friends::friend.eq(user))))
            .select(users::name)
            .get_result::<String>(&*conn)?;
        format!("{}'s wishlist", name)
    };
    let presents = show_presents_for_user(&*conn, me.0, user)?;
    let user_url = format!("/present/add/{}", user);

    Ok((
        owned_html!(
        @if !presents.is_empty() {
            table(border=1) {
                tr {
                    th { : "Present" }
                    @if !you {
                        th { : "Status" }
                    }
                    th { : "Details" }
                    th { : "Delete" }
                }
                @for present in presents {
                    tr {
                        td { : present.short_description }
                        @if !you {
                            @if let Some(gifter) = present.gifter_id {
                                td {
                                    @if gifter == me.0 {
                                        : "Reserved by you";
                                    } else {
                                        : "Reserved by ";
                                        a(href = format!("/user/{}", gifter)) { : present.gifter; }
                                    }
                                }
                            } else {
                                td {
                                    :"Available, click ";
                                    a(href = format!("/present/gift/{}", present.id)) { : "here" }
                                    :" to claim";
                                }
                            }
                        }
                        td {
                            // FIXME: make description not an option anymore
                            a(href = format!("/present/edit/{}", present.id)) { : "Edit" }
                            @if let Some(descr) = present.description {
                                @if !descr.is_empty() {
                                    details {
                                        : descr;
                                    }
                                }
                            }
                        }
                        td {
                            a(href = format!("/present/delete/{}", present.id)) { : "Delete" }
                        }
                    }
                }
            }
        } else {
            : "No presents in your wishlist. Add some to let others know what you want"
        }
        form(action=user_url, method="post") {
            input (name = "short_description", placeholder = "short description");
            button { : "Create new present" }
        }
    ),
        title,
    ))
}

#[get("/<user>")]
pub fn view(
    conn: DbConn,
    me: UserId,
    user: ::geschenke::UserId,
    flash: Option<FlashMessage>,
) -> QueryResult<Content<String>> {
    let (wishlist, title) = print_wishlist(conn, me, user)?;
    Ok(ui::render(&title, flash, wishlist))
}
