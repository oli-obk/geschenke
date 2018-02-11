use rocket::response::Content;
use rocket::http::ContentType;
use horrorshow::prelude::*;
use horrorshow::helper::doctype;
use geschenke::schema::{geschenke, users};
use pool::DbConn;
use diesel::prelude::*;
use diesel::update;
use geschenke::{GeschenkId, Geschenk};
use rocket::request::Form;
use super::UserId;

#[derive(Deserialize, FromForm)]
struct Edit {
    description: String,
}

// FIXME: don't allow editing or viewing presents for anyone but your friends
// FIXME: generalize viewing rules so you can never view presents meant for you

#[post("/edit/<id>", data = "<data>")]
fn edit(conn: DbConn, user: UserId, id: GeschenkId, data: Form<Edit>) -> QueryResult<Content<String>> {
    // FIXME: don't do two queries (one here and one in `edit_view`)
    update(geschenke::table.filter(geschenke::id.eq(id)))
        .set(geschenke::description.eq(&data.get().description))
        .execute(&*conn)?;
    view(conn, user, id)
}

#[get("/edit/<id>")]
fn view(conn: DbConn, user: UserId, id: GeschenkId) -> QueryResult<Content<String>> {
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
                title : &geschenk.short_description;
            }
            body {
                h1 { : geschenk.short_description }
                form(action=format!("{}", id), method="post") {
                    :"The present is for ";
                    a(href=format!("/user/{}", geschenk.receiver)) { :receiver_name } br;
                    :"Description:";
                    input(type="textarea", name="description", value = geschenk.description); br;
                    button { : "Save" }
                }
            }
        }
    ).into_string().unwrap();
    Ok(Content(ContentType::HTML, page))
}
