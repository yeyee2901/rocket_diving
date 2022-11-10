use rocket::async_test;
use rocket::http::Status;
use rocket::local::asynchronous::Client;

#[async_test]
async fn test_endpoint_index() {
    let client = Client::tracked(super::server().await)
        .await
        .expect("failed to launch");

    // do the request
    let response = client
        .get("/")
        .dispatch()
        .await;

    // assert http response
    assert_eq!(response.status(), Status::Ok);

    // assert payload not empty
    assert!(response
        .into_string()
        .await
        .is_some());
}
