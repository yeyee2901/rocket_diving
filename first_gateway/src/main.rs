use rocket::{main as entrypoint, Error};

#[entrypoint]
async fn main() -> Result<(), Error> {
    let _app = first_gateway::server()
        .await
        .launch()
        .await?;

    Ok(())
}
