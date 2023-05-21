use axum;
use reqwest::Client;
use std::sync::Arc;
use std::env;

use cptrain_bot_backend::app::create_app;

#[tokio::main]
async fn main() {
    let client = Arc::new(Client::new());
    let app = create_app(client.clone());
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| {
        eprintln!("Warning: SERVER_PORT environment variable not set. Using default value of 8000.");
        "8000".to_string()
    });
    let server_address = format!("0.0.0.0:{}", server_port);

    match server_address.parse() {
        Ok(addr) => {
            let server = axum::Server::bind(&addr)
                .serve(app.into_make_service());
            println!("CPTrainBot Backend started successfully on {}", addr);
            if let Err(e) = server.await {
                eprintln!("Error running server: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Error parsing server address: {}", e);
        }
    }
}
