#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Fail to bind to port 3000");
    println!("Server running on http://127.0.0.1:3000");

    bench_ai::run(listener)
        .await
        .expect("Fail to run server!"); 
}
