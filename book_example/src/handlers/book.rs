use rocket::{get, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    title: String,
    publisher: String,
    year: u32,
}

#[get("/book")]
pub fn get_book() -> Json<Book> {
    Json(Book {
        title: "Lorem ipsum".to_string(),
        publisher: "set dolor".to_string(),
        year: 2022,
    })
}
