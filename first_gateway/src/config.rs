use rocket::config::Config;
use rocket::figment::Figment;

pub fn load_config() -> Figment {
    Config::figment().merge(("port", 6969))
}
