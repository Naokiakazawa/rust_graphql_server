use dotenvy::dotenv;
use std::env;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    match dotenv() {
        Ok(_) => tracing::info!("Loaded .env file"),
        Err(e) => tracing::error!("Failed to load .env file: {}", e),
    }
    let server_url: String = env::var("SERVER_URL").unwrap();
    let server_port: String = env::var("SERVER_PORT").unwrap();
    println!("Server URL: {}", server_url);
    println!("Server Port: {}", server_port);
}
