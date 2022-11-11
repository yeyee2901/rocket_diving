use rocket::{main as entrypoint, Error};

#[entrypoint]
async fn main() -> Result<(), Error> {
    let _app = book_example::server()
        .await
        .launch()
        .await?;

    Ok(())
}
