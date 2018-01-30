use geschenke::show_presents_for_user;

use horrorshow::prelude::*;
use horrorshow::helper::doctype;
use geschenke::models::User;
use geschenke::schema::users;
use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods};
use pool::DbConn;

use super::UserId;

pub fn hello_user(conn: DbConn, id: UserId) -> QueryResult<String> {
    let user_info = users::table
        .filter(users::id.eq(id.0))
        .get_result::<User>(&*conn)?;
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
            }
        }
    ).into_string().unwrap())
}
