use api::user::print_wishlist;
use diesel::prelude::*;
use geschenke::models::User;
use geschenke::schema::friends;
use geschenke::schema::users;
use pool::DbConn;
use rocket::request::FlashMessage;
use rocket::response::Content;
use ui;

use super::UserId;

pub fn hello_user(
    conn: DbConn,
    id: UserId,
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
    let (wishlist, _) = print_wishlist(conn, id, int_id)?;
    let title = format!("Hello {}!", user_info.name);
    Ok(ui::render(
        &title,
        flash,
        html!(
        h2 { : "Your wishlist" }
        // make this reusable in /user/id
        : wishlist;
        h2 { :"Friends" }
        table {
            @for (friend, id) in friends {
                tr {
                    td {
                        a (href = format!("user/{}", id)) { : friend }
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
            input (name = "email", placeholder = "email address") {}
            button { : "Add friend" }
        }
    ),
    ))
}
