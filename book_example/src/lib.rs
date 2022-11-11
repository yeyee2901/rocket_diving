mod config;
mod handlers;

// external crates
use handlers::book;
use rocket::{routes, Build, Rocket};

#[rocket::launch]
pub async fn server() -> Rocket<Build> {
    rocket::custom(config::load_config()).mount(
        "/api",
        routes![book::get_book],
    )
}
