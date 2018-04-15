use horrorshow::prelude::*;
use horrorshow::helper::doctype;
use rocket::response::Content;
use rocket::http::ContentType;
use rocket::request::FlashMessage;

pub fn render<PAGE: RenderOnce>(
    title: &str,
    flash: Option<FlashMessage>,
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
                a(href="/") { :"Home" }
                : " | ";
                a(href="/account/logout") { :"Logout" }
                h1 { : title }
                :page;
            }
        }
    ).into_string().unwrap();
    Content(ContentType::HTML, page)
}
