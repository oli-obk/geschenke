use rocket::response::Content;
use geschenke::schema::{presents, users};
use pool::DbConn;
use diesel::prelude::*;
use diesel::{update, insert_into, self};
use geschenke::{PresentId, Present, NewPresent};
use rocket::response::{Flash, Redirect};
use rocket::request::{Form, FromFormValue};
use rocket::http::RawStr;
use super::UserId;
use ui;

#[derive(Deserialize, FromForm)]
struct Edit {
    description: String,
}

#[derive(Deserialize, FromForm)]
struct Add {
    short_description: ShortDescription,
}

#[derive(Deserialize)]
struct ShortDescription(String);

impl<'v> FromFormValue<'v> for ShortDescription {
    type Error = ();

    fn from_form_value(form_value: &'v RawStr) -> Result<ShortDescription, ()> {
        let s = form_value.url_decode().map_err(|_| ())?;
        let s = s.trim();
        if s.is_empty() {
            Err(())
        } else {
            Ok(ShortDescription(s.to_owned()))
        }
    }
}

#[post("/add/<recipient>", data = "<data>")]
fn add(conn: DbConn, user: UserId, recipient: ::geschenke::UserId, data: Option<Form<Add>>) -> QueryResult<Flash<Redirect>> {
    if let Some(data) = data {
        let new = NewPresent {
            short_description: &data.get().short_description.0,
            creator: Some(user.0),
            recipient,
        };
        let present = insert_into(presents::table)
            .values(&new)
            .get_result::<Present>(&*conn);
        use diesel::result::{Error, DatabaseErrorKind};
        match present {
            Ok(present) => Ok(Flash::success(Redirect::to(&format!("/present/edit/{}", present.id)), "Added new present")),
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, ref what)) if what.constraint_name() == Some("no_dups_present_short_descriptions") => {
                Ok(Flash::error(Redirect::to(&format!("/user/{}", recipient)), "A present with the same name already exists"))
            }
            Err(other) => Err(other),
        }
    } else {
        Ok(Flash::error(Redirect::to(&format!("/user/{}", recipient)), "Cannot add a present without a description"))
    }
}

// FIXME: don't allow adding/editing presents for anyone but your friends

#[post("/edit/<id>", data = "<data>")]
fn edit(conn: DbConn, user: UserId, id: PresentId, data: Form<Edit>) -> QueryResult<Content<String>> {
    let present = update(presents::table.filter(presents::id.eq(id)))
        .set(presents::description.eq(&data.get().description))
        .get_result::<Present>(&*conn)?;
    render(conn, user, present)
}

#[get("/delete/<id>")]
fn delete(conn: DbConn, _user: UserId, id: PresentId) -> QueryResult<Flash<Redirect>> {
    let present = diesel::delete(presents::table.filter(presents::id.eq(id)))
        .get_result::<Present>(&*conn)?;
    Ok(Flash::success(Redirect::to(&format!("/user/{}", present.recipient)), "Deleted present"))
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
