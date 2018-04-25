use fluent::MessageContext;
use fluent::types::FluentValue;
use std::collections::HashMap;

use rocket::outcome::IntoOutcome;
use rocket::request::{self, Request, FromRequest};

use accept_language::parse;

use rocket::State;

impl<'a> Lang<'a> {
    pub fn format(
        &self, 
        id: &str,
        args: Option<&HashMap<&str, FluentValue>>
    ) -> String {
        for ctx in &self.ctx {
            if let Some(msg) = ctx.get_message(id) {
                if let Some(result) = ctx.format(msg, args) {
                    return result;
                }
            }
        }
        panic!("no localization at all for {:?}", id);
    }
}

pub struct Lang<'a> {
    // contexts ordered in the preferred user order
    ctx: Vec<&'a MessageContext<'static>>,
}

impl<'a, 'r> FromRequest<'a, 'r> for Lang<'r> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Lang<'r>, ()> {
        let langs = request.guard::<State<'r, Langs>>()?.inner();
        let mut ctx = Vec::new();
        for lang in request.headers().get("Accept-Language") {
            for lang in parse(lang) {
                if let Some(lang) = langs.get(&*lang) {
                    ctx.push(lang);
                }
            }
        }
        // add a final fallback
        ctx.push(&langs["en-US"]);
        Some(Lang { ctx }).or_forward(())
    }
}

pub type Langs = HashMap<&'static str, MessageContext<'static>>;

pub fn load() -> Langs {
    let mut ctx = HashMap::new();

    macro_rules! lang {
        ($name:expr) => {{
            let mut lang = MessageContext::new(&[$name]);
            lang.add_messages(include_str!(concat!("../../localization/", $name)));
            ctx.insert($name, lang);
        }};
    }
    lang!("en-US");
    lang!("de-DE");
    ctx
}
