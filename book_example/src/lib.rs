mod config;
mod handlers;

// external crates
use handlers::book;
use rocket::{routes, Build, Rocket};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Swagger documentations are declared here using proc-macro
#[derive(OpenApi)]
#[openapi(
    paths(
        book::get_book,
        book::create_book,
    ),
    components(
        schemas(
            book::model::Book,
            book::model::ReqCreateBook,
            book::model::RespCreateBook
        )
    ),
    tags(
        (name = "book", description = "Book related requests")
    )
)]
struct ApiDoc;

#[rocket::launch]
pub async fn server() -> Rocket<Build> {
    // load config file
    let app_cfg = config::load_config();

    // build the app
    rocket::custom(app_cfg)
        // mount swagger UI
        .mount(
            "/",
            SwaggerUi::new("/swagger/<_..>").url(
                "/doc/openapi.json",
                ApiDoc::openapi(),
            ),
        )
        // populate the endpoints
        .mount(
            "/api",
            routes![book::get_book, book::create_book],
        )
}
