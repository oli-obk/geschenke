use horrorshow::prelude::*;
use horrorshow::helper::doctype;
use rocket::response::Content;
use rocket::http::ContentType;

pub fn render<PAGE: RenderOnce>(
    title: &str,
    page: PAGE,
) -> Content<String> {
    let page = html!(
        : doctype::HTML;
        html {
            head {
                title : title
            }
            body {
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
