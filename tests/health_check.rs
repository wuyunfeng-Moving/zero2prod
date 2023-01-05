use std::net::TcpListener;

use actix_web::web::resource;

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();

    println!("{}", &addr);
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/health_check", &addr))
        .send()
        .await
        .expect("failed to execute request!");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let tcplistener = TcpListener::bind("127.0.0.1:0").unwrap();

    let addr_str = tcplistener.local_addr().unwrap();

    let server = ztp::run(tcplistener).expect("failed to bind address");

    let _ = tokio::spawn(server);

    addr_str.to_string()
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("http://{}/subscriptions", &app_address))
        .header("Content-Type", "application/X-WWW-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to executed the request!");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name = le%20guin", "missing the email!"),
        ("email=ursula_le_guin%40gmail.com", "missing the name!"),
        ("", "missing both name and email!"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("http://{}/subscriptions", &app_address))
            .header("Content-Type", "application/X-WWW-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to executed request!");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad request when the payload was {}",
            error_message
        );
    }
}
