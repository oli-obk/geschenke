use geschenke::show_presents_for_user;

use horrorshow::prelude::*;
use horrorshow::helper::doctype;
use geschenke::models::User;
use geschenke::schema::users;
use geschenke::schema::friends;
use diesel::prelude::*;
use pool::DbConn;
use rocket::request::FlashMessage;

use super::UserId;

pub fn hello_user(conn: DbConn, id: UserId, flash: Option<FlashMessage>) -> QueryResult<String> {
    let user_info = users::table
        .filter(users::id.eq(id.0))
        .get_result::<User>(&*conn)?;
    let friends = users::table
        .inner_join(friends::table.on(friends::friend.eq(users::id)))
        .filter(friends::id.eq(id.0))
        .select((users::name, users::id))
        .load::<(String, ::geschenke::UserId)>(&*conn)?;
    // presents I created for myself
    let geschenke = show_presents_for_user(&*conn, id.0, id.0)?;
    Ok(html!(
        : doctype::HTML;
        html {
            head {
                title : format!("Hello {}!", user_info.name);
            }
            body {
                form(action="account/logout", method="post") {
                    button { : "Logout" }
                }
                br;
                @if let Some(flash) = flash {
                    span (style = flash.name()) {: flash.msg() }
                    br;
                }
                h1 { : "Presents" }
                table {
                    tr {
                        td {
                            : "Description"
                        }
                        td {
                            : "Edit"
                        }
                    }
                    @ for geschenk in geschenke {
                        tr {
                            td {
                                : geschenk.short_description
                            }
                            td {
                                a(href = format!("geschenk/edit/{}", geschenk.id)) { : "Edit" }
                            }
                        }
                    }
                }
                h1 { :"Friends" }
                table {
                    @for (friend, id) in friends {
                        tr {
                            td {
                                : friend
                            }
                            td {
                                form(action="user/friend/remove", method="post") {
                                    input (name = "id", value = id, type = "hidden");
                                    button { : "Delete" }
                                }
                            }
                        }
                    }
                }
                form(action="user/friend/add", method="post") {
                    input (name = "email") {}
                    button { : "Add friend via email address" }
                }
            }
        }
    ).into_string().unwrap())
}
