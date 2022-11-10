use rocket::get;

#[get("/")]
pub fn handle() -> &'static str {
    "Hello, Rocket!"
}
