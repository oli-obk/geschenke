use horrorshow::prelude::*;
use horrorshow::helper::doctype;

pub fn render<PAGE: RenderOnce>(
    title: &str,
    page: PAGE,
) -> String {
    html!(
        : doctype::HTML;
        html {
            head {
                title : title
            }
            body {
                h1 { : title }
                :page;
            }
        }
    ).into_string().unwrap()
}
