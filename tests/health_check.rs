use std::net::TcpListener;

// !tests/health_check.rs

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener)
        .await
        .expect("Failed to bind address.");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to send request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_web::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();
    let body = "name=pro%20dev&email=pro%40dev.com";

    let response = client
    .post(&format!("{}/subscriptions", address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16())
}

#[actix_web::test]
async fn subscribe_returns_400_when_form_data_missing() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=pro%20dev", "missing the email"),
        ("email=pro%40dev.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (body, message) in test_cases {
        let response = client
        .post(&format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with 400 Bad Request when the payload was {}", message
        );        
    }
}
