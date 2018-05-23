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
                Err(err) => return user_creation_error(err, lang),
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
                lang.format("friend-self-hugging", None),
            ))
        }
    }
    // if we didn't have the friend, we do have them now
    // try adding the reverse friendship, but ignore if already exists
    try(user.0, friend)?;

    Ok(Flash::success(Redirect::to("/"), lang.format("added-friend", None)))
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

pub fn print_wishlist<'a>(
    conn: DbConn,
    me: UserId,
    user: ::geschenke::UserId,
    lang: Lang<'a>
) -> QueryResult<(impl RenderOnce + 'a, String)> {
    let you = me.0 == user;
    let title = if you {
        lang.format("wishlist", None).to_owned()
    } else {
        let name = user_name(&*conn, me, user)?;
        lang.format(
            "someones-wishlist", 
            fluent_map!{
                "name" => name,
            }
        )
    };
    let presents = show_presents_for_user(&*conn, me.0, user)?;
    let user_url = format!("/present/add/{}", user);

    Ok((
        owned_html!(
        @if !presents.is_empty() {
            table(border=1) {
                tr {
                    th { : lang.format("present", None) }
                    @if !you {
                        th { : lang.format("status", None) }
                    }
                    th { : lang.format("details", None) }
                    th { : lang.format("delete", None) }
                }
                @for present in presents {
                    tr {
                        td { : present.short_description }
                        @if !you {
                            @if let Some(gifter) = present.gifter_id {
                                td {
                                    @if gifter == me.0 {
                                        : lang.format("reserved-by-you", None); : ", "; : lang.format("click", None); : " ";
                                        a(href = format!("/present/free/{}", present.id)) { : lang.format("here", None) }
                                        : " "; : lang.format("unreserve", None);
                                    } else {
                                        : lang.format("reserved-by", None); : " ";
                                        a(href = format!("/user/{}", gifter)) { : present.gifter; }
                                    }
                                }
                            } else {
                                td {
                                    : lang.format("available", None); : ", "; : lang.format("click", None); : " ";
                                    a(href = format!("/present/gift/{}", present.id)) { : lang.format("here", None) }
                                    : " "; : lang.format("claim", None);
                                }
                            }
                        }
                        td {
                            // FIXME: make description not an option anymore
                            a(href = format!("/present/edit/{}", present.id)) { : lang.format("edit", None) }
                            @if let Some(descr) = present.description {
                                @if !descr.is_empty() {
                                    details {
                                        summary { : lang.format("show-description", None);}
                                        : descr;
                                    }
                                }
                            }
                        }
                        td {
                            a(href = format!("/present/delete/{}", present.id)) { : lang.format("delete", None) }
                        }
                    }
                }
            }
        } else {
            : lang.format("no-presents", None)
        }
        form(action=user_url, method="post") {
            input (name = "short_description", placeholder = lang.format("short-description", None));
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
    let (wishlist, title) = print_wishlist(conn, me, user, lang.clone())?;
    Ok(ui::render(&title, flash, lang, wishlist))
}

#[get("/friend/custom-add")]
pub fn custom_add_friend(
    flash: Option<FlashMessage>,
    lang: Lang,
) -> QueryResult<Content<String>> {
    let name_msg = lang.format("name", None);
    let email_msg = lang.format("mail", None);
    Ok(ui::render(
        &lang.format("custom-add-friend-title", None),
        flash,
        lang.clone(),
        html!(
            h2 { : lang.format("custom-add-friend-title", None) }
            form(action="user/friend/custom-add", method="post") {
                textarea (
                    name = "body",
                    placeholder = lang.format("mail_body", None),
                    required,
                    wrap = "soft",
                    cols = "50",
                    rows = "10"
                ) {}
                table {
                    tr {
                        td { : &name_msg; }
                        td { : &email_msg; }
                    }
                    @for i in 0..20 {
                        tr {
                            td {
                                input (name = format!("name{}", i), placeholder = &name_msg) {}
                            }
                            td {
                                input (type = format!("email{}", i), name = "email", placeholder = &email_msg) {}
                            }
                        }
                    }
                }
                button { : lang.format("add-friends", None) }
            }
        ),
    ))
}
