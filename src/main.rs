use rocket_diving;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let app = rocket_diving::server().await;

    let _app = app.launch().await?;

    Ok(())
}
