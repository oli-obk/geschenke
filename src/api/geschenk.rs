use rocket::response::Content;
use geschenke::schema::{geschenke, users};
use pool::DbConn;
use diesel::prelude::*;
use diesel::{update, insert_into};
use geschenke::{GeschenkId, Geschenk, NewGeschenk};
use rocket::request::Form;
use super::UserId;
use ui;

#[derive(Deserialize, FromForm)]
struct Edit {
    description: String,
}

#[derive(Deserialize, FromForm)]
struct Add {
    short_description: String,
}

#[post("/add/<receiver>", data = "<data>")]
fn add(conn: DbConn, user: UserId, receiver: ::geschenke::UserId, data: Form<Add>) -> QueryResult<Content<String>> {
    let new = NewGeschenk {
        short_description: &data.get().short_description,
        creator: Some(user.0),
        receiver,
    };
    let geschenk = insert_into(geschenke::table)
        .values(&new)
        .get_result::<Geschenk>(&*conn)?;
    render(conn, user, geschenk)
}

// FIXME: don't allow adding/editing presents for anyone but your friends

#[post("/edit/<id>", data = "<data>")]
fn edit(conn: DbConn, user: UserId, id: GeschenkId, data: Form<Edit>) -> QueryResult<Content<String>> {
    let geschenk = update(geschenke::table.filter(geschenke::id.eq(id)))
        .set(geschenke::description.eq(&data.get().description))
        .get_result::<Geschenk>(&*conn)?;
    render(conn, user, geschenk)
}

#[get("/edit/<id>")]
fn view(conn: DbConn, user: UserId, id: GeschenkId) -> QueryResult<Content<String>> {
    let geschenk = ::geschenke::get_present(&*conn, user.0, id)?;
    render(conn, user, geschenk)
}

fn render(conn: DbConn, user: UserId, geschenk: Geschenk) -> QueryResult<Content<String>> {
    let receiver_name = if geschenk.receiver == user.0 {
        "You".to_string()
    } else {
        users::table
            .filter(users::id.eq(geschenk.receiver))
            .select(users::name)
            .get_result::<String>(&*conn)?
    };
    let Geschenk {
        short_description,
        id,
        receiver,
        description,
        ..
    } = geschenk;
    Ok(ui::render(&short_description, html!(
        form(action=format!("/geschenk/edit/{}", id), method="post") {
            :"The present is for ";
            a(href=format!("/user/{}", receiver)) { :receiver_name } br;
            :"Description:";
            input(type="textarea", name="description", value = description); br;
            button { : "Save" }
        }
    )))
}
