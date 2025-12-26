use reqwest;
use std::time::Duration;
use tokio::time::sleep;

// Helper to start the server in the background
async fn spawn_app() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    tokio::spawn(async move {
        bench_ai::run(listener)
            .await
            .expect("Failed to run server");
    });
    // Give the server a moment to start
    sleep(Duration::from_millis(100)).await;
    address
}

#[tokio::test]
async fn health_check_works() {
    //Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health", address))
        .send()
        .await
        .expect("Fail to execute request");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);

    let body = response.text().await.expect("Failed to read response body");
    assert!(body.contains("ok"));
}