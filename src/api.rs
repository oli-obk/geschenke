
#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}
