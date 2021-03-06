use horrorshow::helper::doctype;
use horrorshow::prelude::*;
use rocket::http::ContentType;
use rocket::request::FlashMessage;
use rocket::response::Content;

use chrono::prelude::*;
use email_format::Email;
use mail::Mail;
use rocket::State;
use ui::localization::Lang;
use std::collections::HashMap;
use fluent::types::FluentValue;
use asciifier::asciifier::Asciifier;

#[macro_use]
pub mod localization;

pub fn send_mail(
    mailstrom: &State<Mail>,
    lang: Lang,
    email_address: &str,
    caption: &str,
    id: &str,
    args: Option<HashMap<&str, FluentValue>>,
) {
    let now: DateTime<Utc> = Utc::now();
    let mut email = Email::new(
        "geschenke@oli-obk.de", // "From:"
        &now,                   // "Date:"
    ).unwrap();

    email.set_sender("geschenke@oli-obk.de").unwrap();
    email.set_to(email_address).unwrap();
    email.set_subject(caption).unwrap();
    let asciifier = Asciifier::new();
    let body = asciifier.to_ascii(
        lang.format(id, args).replace('\r', "").replace('\n', "\r\n")
    );
    email.set_body(&*body).unwrap();

    mailstrom.lock().unwrap().send_email(email).unwrap();
}

pub fn render<PAGE: RenderOnce>(
    title: &str,
    flash: Option<FlashMessage>,
    lang: Lang,
    page: PAGE,
) -> Content<String> {
    let page = html!(
        : doctype::HTML;
        html {
            head {
                title : title
            }
            body {
                @if let Some(flash) = flash {
                    span (style = flash.name()) {: flash.msg() }
                    br;
                }
                a(href="/") { : lang.format("home", None) }
                : " | ";
                a(href="/account/logout") { : lang.format("logout", None) }
                h1 { : title }
                :page;
            }
        }
    ).into_string()
        .unwrap();
    Content(ContentType::HTML, page)
}
