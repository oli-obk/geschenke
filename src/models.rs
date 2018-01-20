use chrono::NaiveDateTime as DateTime;
use super::schema::{users, geschenke};
use UserId;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub autologin: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub autologin: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct Geschenk {
    pub id: i32,
    pub short_description: String,
    pub description: Option<String>,
    pub creator: Option<UserId>,
    pub receiver: UserId,
    pub gifter: Option<UserId>,
    pub obtained_date: Option<DateTime>,
    pub gifted_date: Option<DateTime>,
}

#[derive(Insertable)]
#[table_name="geschenke"]
pub struct NewGeschenk<'a> {
    pub short_description: &'a str,
    pub description: &'a str,
    pub creator: Option<UserId>,
    pub receiver: UserId,
}
