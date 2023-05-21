use axum::{extract::Path , routing::get , Router};
use reqwest::Client;
use std::sync::Arc;

use crate::handlers::train_handler::handler;

pub fn train_route(client: Arc<Client>) -> Router {
    Router::new().route(
        "/train/:train_number",
        get({
            move |path: Path<i32>| handler(client.clone(), path)
        }),
    )
}
