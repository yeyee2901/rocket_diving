use rocket::get;

#[get("/")]
pub fn handle<'r>() -> &'r str {
    "Hello world"
}
