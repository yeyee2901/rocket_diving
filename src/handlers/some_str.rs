use rocket::get;

#[get("/some_str?<name>")]
pub fn handle(name: &str) -> String {
    String::from(name)
}
