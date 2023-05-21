use axum::Router;
use reqwest::Client;
use std::sync::Arc;

use crate::routes::train::train_route;

pub fn create_app(client: Arc<Client>) -> Router {
    Router::new()
        .nest("/", train_route(client.clone()))
}
