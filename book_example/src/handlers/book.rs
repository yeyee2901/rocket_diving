use rocket::serde::json::Json;
use rocket::{get, post};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

/// Book Models are defined here
pub mod model {
    use super::*;

    #[derive(Serialize, Deserialize, ToSchema)]
    pub struct Book {
        /// title of the book
        pub title: String,

        /// publisher of the book
        pub publisher: String,

        /// publication year
        pub year: u32,
    }

    #[derive(
        Serialize, Deserialize, ToSchema, Validate,
    )]
    #[validate(schema(function = "validate_req_create_book"))]
    pub struct ReqCreateBook {
        /// new book title
        #[schema(example = "The Rust Book")]
        pub title: String,
    }

    #[derive(Serialize, ToSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct RespCreateBook {
        /// response code
        pub response_code: String,

        /// response message, explaining operation error / success details
        pub response_message: String,

        /// data, the data scheme should match the context of the endpoint
        pub data: Book,
    }


    /// validate book creation on "POST /api/book"
    pub fn validate_req_create_book(
        req: &ReqCreateBook,
    ) -> Result<(), ValidationError> {
        match req.title.is_empty() {
            false => Ok(()),
            true => Err(ValidationError::new(
                "Title must be non empty string",
            )),
        }
    }
}

/// Get a book
#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "get single book", body = [Book]),
    )
)]
#[get("/book")]
pub fn get_book() -> Json<model::Book> {
    Json(model::Book {
        title: "this is edited via rebasing".to_string(),
        publisher: "this is edited via reset --soft".to_string(),
        year: 2022,
    })
}

/// Insert a new book
#[utoipa::path(
    context_path = "/api",
    request_body = ReqCreateBook,
    responses(
        (status = 200, description = "Success", body = [RespCreateBook]),
        (status = 400, description = "Bad Request", body = [RespCreateBook]),
    )
)
]
#[post("/book", data = "<new_book>")]
pub async fn create_book(
    new_book: Json<model::ReqCreateBook>,
) -> Json<model::RespCreateBook> {
    let req = new_book.into_inner();

    match req.validate() {
        Ok(_) => {
            return Json(model::RespCreateBook {
                response_code: "200".to_string(),
                response_message: "success".to_string(),
                data: model::Book {
                    title: req.title,
                    publisher: "The JSON validator".to_string(),
                    year: 2022,
                },
            })
        }
        Err(err) => {
            return Json(model::RespCreateBook {
                response_code: "400".to_string(),
                response_message: format!("Bad Request. {}", err).to_string(),
                data: model::Book {
                    title: req.title,
                    publisher: "The JSON validator".to_string(),
                    year: 2022,
                },
            })
        }
    }
}
