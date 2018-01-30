use rocket::response::Content;
use rocket::http::ContentType;
use horrorshow::prelude::*;
use horrorshow::helper::doctype;
use geschenke::schema::{geschenke, users};
use pool::DbConn;
use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods};
use geschenke::{GeschenkId, Geschenk};
use super::UserId;

#[get("/edit/<id>")]
fn edit(conn: DbConn, user: UserId, id: GeschenkId) -> QueryResult<Content<String>> {
    let geschenk = geschenke::table
        .filter(geschenke::id.eq(id))
        .get_result::<Geschenk>(&*conn)?;
    let receiver_name = if geschenk.receiver == user.0 {
        "You".to_string()
    } else {
        users::table
            .filter(users::id.eq(geschenk.receiver))
            .select(users::name)
            .get_result::<String>(&*conn)?
    };
    /*if geschenk.creator != Some(user.0) {
        return ;
    }*/
    let page = html!(
        : doctype::HTML;
        html {
            head {
                title : geschenk.short_description.clone();
            }
            body {
                h1 { : geschenk.short_description }
                form(action="geschenk/edit", method="post") {
                    :"The present is for ";
                    a(href=format!("/user/{}", geschenk.receiver)) { :receiver_name } br;
                    :"Long Description:";
                    input(type="textarea", name="long_description", value = geschenk.description); br;
                    button { : "Save" }
                }
            }
        }
    ).into_string().unwrap();
    Ok(Content(ContentType::HTML, page))
}
