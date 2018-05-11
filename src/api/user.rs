use super::UserId;
use diesel::prelude::*;
use diesel::{delete, insert_into};
use geschenke::models::NewFriend;
use geschenke::schema::{friends, users};
use geschenke::show_presents_for_user;
use api::registration::{try_create_user, User, user_creation_error};
use horrorshow::RenderOnce;
use pool::DbConn;
use rocket::request::FlashMessage;
use rocket::request::Form;
use rocket::response::Content;
use rocket::response::{Flash, Redirect};
use ui;
use rocket::State;
use mail::Mail;
use ui::localization::Lang;

#[derive(Deserialize, FromForm)]
pub struct RemoveUser {
    id: ::geschenke::UserId,
}

#[post("/friend/add", data = "<new_friend>")]
pub fn add_friend(
    conn: DbConn,
    user: UserId,
    mailstrom: State<Mail>,
    lang: Lang,
    new_friend: Form<User>,
) -> QueryResult<Flash<Redirect>> {
    let friend = users::table
        .filter(users::email.eq(&new_friend.get().email))
        .select(users::id)
        .get_result::<::geschenke::UserId>(&*conn)
        .optional()?;
    let friend = match friend {
        Some(friend) => friend,
        None => {
            match try_create_user(&*conn, &new_friend.get()) {
                Ok((id, autologin)) => {
                    ui::send_mail(
                        mailstrom,
                        lang.clone(),
                        &new_friend.get().email,
                        &lang.format("invitation-subject", None),
                        "invite-mail",
                        fluent_map!{
                            "email_address" => new_friend.get().email.clone(),
                            "autologin" => autologin,
                            "name" => new_friend.get().name.clone(),
                            "forward" => format!("/user/{}", user.0),
                            "who" => my_user_name(&*conn, user)?,
                        },
                    );
                    id
                },
                Err(err) => return user_creation_error(err),
            }
        }
    };
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
        Info::Already => return Ok(Flash::error(Redirect::to("/"), lang.format("already-friends", None))),
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

    Ok(Flash::error(Redirect::to("/"), lang.format("added-friend", None)))
}

#[post("/friend/remove", data = "<delete_friend>")]
pub fn remove_friend(
    conn: DbConn,
    user: UserId,
    delete_friend: Form<RemoveUser>,
    lang: Lang,
) -> QueryResult<Flash<Redirect>> {
    let id = friends::id.eq(user.0);
    let friend_id = friends::friend.eq(delete_friend.get().id);
    let query = friends::table.filter(id.and(friend_id));
    if delete(query).execute(&*conn).is_ok() {
        Ok(Flash::success(Redirect::to("/"), lang.format("deleted-friend", None)))
    } else {
        Ok(Flash::error(Redirect::to("/"), "Could not delete friend"))
    }
}

fn my_user_name(
    conn: &PgConnection,
    me: UserId,
) -> QueryResult<String> {
    users::table
        .filter(users::id.eq(me.0))
        .select(users::name)
        .get_result::<String>(conn)
}

fn user_name(
    conn: &PgConnection,
    me: UserId,
    user: ::geschenke::UserId,
) -> QueryResult<String> {
    users::table
        .filter(users::id.eq(user))
        .inner_join(friends::table.on(friends::id.eq(me.0).and(friends::friend.eq(user))))
        .select(users::name)
        .get_result::<String>(conn)
}

pub fn print_wishlist(
    conn: DbConn,
    me: UserId,
    user: ::geschenke::UserId,
    lang: Lang
) -> QueryResult<(impl RenderOnce, String)> {
    let you = me.0 == user;
    let title = if you {
        "Your wishlist".to_owned()
    } else {
        let name = user_name(&*conn, me, user)?;
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
                                        : "Reserved by you. Click ";
                                        a(href = format!("/present/free/{}", present.id)) { : "here" }
                                        : " to unreserve";
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
            button { : lang.format("create-present", None) }
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
    lang: Lang,
) -> QueryResult<Content<String>> {
    let (wishlist, title) = print_wishlist(conn, me, user, lang)?;
    Ok(ui::render(&title, flash, lang, wishlist))
}
