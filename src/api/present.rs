use rocket::response::Content;
use geschenke::schema::{presents, users};
use pool::DbConn;
use diesel::prelude::*;
use diesel::{update, insert_into};
use geschenke::{PresentId, Present, NewPresent};
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

#[post("/add/<recipient>", data = "<data>")]
fn add(conn: DbConn, user: UserId, recipient: ::geschenke::UserId, data: Form<Add>) -> QueryResult<Content<String>> {
    let new = NewPresent {
        short_description: &data.get().short_description,
        creator: Some(user.0),
        recipient,
    };
    let present = insert_into(presents::table)
        .values(&new)
        .get_result::<Present>(&*conn)?;
    render(conn, user, present)
}

// FIXME: don't allow adding/editing presents for anyone but your friends

#[post("/edit/<id>", data = "<data>")]
fn edit(conn: DbConn, user: UserId, id: PresentId, data: Form<Edit>) -> QueryResult<Content<String>> {
    let present = update(presents::table.filter(presents::id.eq(id)))
        .set(presents::description.eq(&data.get().description))
        .get_result::<Present>(&*conn)?;
    render(conn, user, present)
}

#[get("/edit/<id>")]
fn view(conn: DbConn, user: UserId, id: PresentId) -> QueryResult<Content<String>> {
    let present = ::geschenke::get_present(&*conn, user.0, id)?;
    render(conn, user, present)
}

fn render(conn: DbConn, user: UserId, present: Present) -> QueryResult<Content<String>> {
    let recipient_name = if present.recipient == user.0 {
        "You".to_string()
    } else {
        users::table
            .filter(users::id.eq(present.recipient))
            .select(users::name)
            .get_result::<String>(&*conn)?
    };
    let Present {
        short_description,
        id,
        recipient,
        description,
        ..
    } = present;
    Ok(ui::render(&short_description, None, html!(
        form(action=format!("/present/edit/{}", id), method="post") {
            :"The present is for ";
            a(href=format!("/user/{}", recipient)) { :recipient_name } br;
            :"Description:";
            input(type="textarea", name="description", value = description); br;
            button { : "Save" }
        }
    )))
}
