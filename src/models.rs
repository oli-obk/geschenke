use chrono::NaiveDateTime as DateTime;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Queryable)]
pub struct Geschenk {
    pub id: i32,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub creator: Option<i32>,
    pub receiver: i32,
    pub gifter: Option<i32>,
    pub obtained_date: Option<DateTime>,
    pub gifted_date: Option<DateTime>,
}
