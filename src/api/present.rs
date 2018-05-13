use super::UserId;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::{self, insert_into, update};
use geschenke::schema::{presents, users};
use geschenke::{NewPresent, Present, PresentId};
use pool::DbConn;
use rocket::http::RawStr;
use rocket::request::{Form, FromFormValue};
use rocket::response::Content;
use rocket::response::{Flash, Redirect};
use ui;
use ui::localization::Lang;

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
fn add(
    conn: DbConn,
    user: UserId,
    recipient: ::geschenke::UserId,
    lang: Lang,
    data: Option<Form<Add>>,
) -> QueryResult<Flash<Redirect>> {
    if let Some(data) = data {
        let new = NewPresent {
            short_description: &data.get().short_description.0,
            creator: Some(user.0),
            recipient,
        };
        let present = insert_into(presents::table)
            .values(&new)
            .get_result::<Present>(&*conn);
        use diesel::result::{DatabaseErrorKind, Error};
        match present {
            Ok(present) => Ok(Flash::success(
                Redirect::to(&format!("/present/edit/{}", present.id)),
                lang.format("added-present", None),
            )),
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, ref what))
                if what.constraint_name() == Some("no_dups_present_short_descriptions") =>
            {
                Ok(Flash::error(
                    Redirect::to(&format!("/user/{}", recipient)),
                    lang.format("present-exists", None),
                ))
            }
            Err(other) => Err(other),
        }
    } else {
        Ok(Flash::error(
            Redirect::to(&format!("/user/{}", recipient)),
            lang.format("adding-without-description", None),
        ))
    }
}

// FIXME: don't allow adding/editing presents for anyone but your friends

#[post("/edit/<id>", data = "<data>")]
fn edit(
    conn: DbConn,
    _user: UserId,
    id: PresentId,
    lang: Lang,
    data: Form<Edit>,
) -> QueryResult<Flash<Redirect>> {
    let present = update(presents::table.filter(presents::id.eq(id)))
        .set(presents::description.eq(&data.get().description))
        .get_result::<Present>(&*conn)?;
    Ok(Flash::success(
        Redirect::to(&format!("/user/{}", present.recipient)),
        lang.format("present-saved", None),
    ))
}

#[get("/delete/<id>")]
fn delete(conn: DbConn, _user: UserId, id: PresentId, lang: Lang) -> QueryResult<Flash<Redirect>> {
    let present =
        diesel::delete(presents::table.filter(presents::id.eq(id))).get_result::<Present>(&*conn)?;
    Ok(Flash::success(
        Redirect::to(&format!("/user/{}", present.recipient)),
        lang.format("present-deleted", None),
    ))
}

#[get("/gift/<id>")]
fn gift(conn: DbConn, user: UserId, id: PresentId, lang: Lang) -> QueryResult<Flash<Redirect>> {
    let now: NaiveDateTime = Utc::now().naive_utc();
    let present = diesel::update(
        presents::table.filter(presents::id.eq(id).and(presents::recipient.ne(user.0))),
    ).set((
        presents::reserved_date.eq(Some(now)),
        presents::gifter.eq(Some(user.0)),
    ))
        .get_result::<Present>(&*conn)?;
    Ok(Flash::success(
        Redirect::to(&format!("/user/{}", present.recipient)),
        lang.format("info-gifting", None),
    ))
}

#[get("/free/<id>")]
fn free(conn: DbConn, user: UserId, id: PresentId, lang: Lang) -> QueryResult<Flash<Redirect>> {
    let present = diesel::update(
        presents::table.filter(presents::id.eq(id).and(presents::recipient.ne(user.0))),
    ).set((
        presents::reserved_date.eq(None::<NaiveDateTime>),
        presents::gifter.eq(None::<i32>),
    ))
        .get_result::<Present>(&*conn)?;
    Ok(Flash::success(
        Redirect::to(&format!("/user/{}", present.recipient)),
        lang.format("info-not-gifting", None),
    ))
}

#[get("/edit/<id>")]
fn view(conn: DbConn, user: UserId, id: PresentId, lang: Lang) -> QueryResult<Content<String>> {
    let present = ::geschenke::get_present(&*conn, user.0, id)?;
    render(conn, user, present, lang)
}

fn render(conn: DbConn, user: UserId, present: Present, lang: Lang) -> QueryResult<Content<String>> {
    let you = present.recipient == user.0;
    let recipient_name = if you {
        lang.format("you-akkusativ", None).to_string()
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
        reserved_date,
        gifter,
        ..
    } = present;
    let gifter = match gifter {
        Some(gifter) => Some(users::table
            .filter(users::id.eq(gifter))
            .select(users::name)
            .get_result::<String>(&*conn)?),
        None => None,
    };
    let recipient = html!(a(href=format!("/user/{}", recipient)) { :&recipient_name });
    Ok(ui::render(
        &short_description,
        None,
        lang.clone(),
        html!(
        form(action=format!("/present/edit/{}", id), method="post") {
            :lang.format("present-for", None);
            :&recipient; br;
            @if !you {
                @if let Some(reserved_date) = reserved_date {
                    :lang.format(
                        "present-reserved",
                        fluent_map!{
                            "date" => reserved_date.format("%Y-%m-%d").to_string(),
                            "gifter" => gifter.unwrap(),
                        }
                    );
                    : recipient;
                }
            }
            br;
            :lang.format("description", None); : ":";
            input(type="textarea", name="description", value = description); br;
            button { : lang.format("save", None) }
        }
    ),
    ))
}
