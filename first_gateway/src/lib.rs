// internal modules
mod config;
mod handlers;

// external crates
use rocket::*;

// namespace shortening
use handlers::*;

#[launch]
pub async fn server() -> Rocket<Build> {
    rocket::custom(config::load_config())
        .mount("/", routes![index::handle,])
}

#[cfg(test)]
mod test;
