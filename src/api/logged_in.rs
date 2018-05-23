use api::user::print_wishlist;
use diesel::prelude::*;
use geschenke::models::User;
use geschenke::schema::friends;
use geschenke::schema::users;
use pool::DbConn;
use rocket::request::FlashMessage;
use rocket::response::Content;
use ui;
use ui::localization::Lang;

use super::UserId;

pub fn hello_user(
    conn: DbConn,
    id: UserId,
    lang: Lang,
    flash: Option<FlashMessage>,
) -> QueryResult<Content<String>> {
    let int_id = id.0;
    let user_info = users::table
        .filter(users::id.eq(int_id))
        .get_result::<User>(&*conn)?;
    let friends = users::table
        .inner_join(friends::table.on(friends::friend.eq(users::id)))
        .filter(friends::id.eq(int_id))
        .select((users::name, users::id))
        .load::<(String, ::geschenke::UserId)>(&*conn)?;
    let (wishlist, _) = print_wishlist(conn, id, int_id, lang.clone())?;
    let title = lang.format(
        "intro",
        fluent_map!{
            "name" => user_info.name,
        }
    );
    Ok(ui::render(
        &title,
        flash,
        lang.clone(),
        html!(
        h2 { : lang.format("wishlist", None) }
        // make this reusable in /user/id
        : wishlist;
        h2 { : lang.format("friends", None) }
        table {
            @for (friend, id) in friends {
                tr {
                    td {
                        a (href = format!("user/{}", id)) { : friend }
                    }
                    td {
                        form(action="user/friend/remove", method="post") {
                            input (name = "id", value = id, type = "hidden");
                            button { : lang.format("delete", None) }
                        }
                    }
                }
            }
        }
        form(action="user/friend/add", method="post") {
            input (name = "name", placeholder = lang.format("name", None)) {}
            input (type = "email", name = "email", placeholder = lang.format("mail", None)) {}
            button { : lang.format("add-friend", None) }
        }
        a(href="user/friend/custom-add", method="get") {
            : lang.format("custom-add-friend", None);
        }
    ),
    ))
}
