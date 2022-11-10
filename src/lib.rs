mod config;
mod handlers;

use rocket::{get, launch, routes, Build, Rocket};

use handlers::some_str;

#[get("/")]
fn index() -> String {
    "Hello world!".to_string()
}

#[launch]
pub async fn server() -> Rocket<Build> {
    rocket::custom(config::load_config()).mount("/", routes![index, some_str::handle])
}
