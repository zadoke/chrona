use axum::Router;
use axum::http::Method;
use reqwest::Client;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use crate::routes::train::train_route;
use crate::routes::station::station_search_route;
use crate::routes::schedule::station_arrival_route;
use crate::routes::schedule::station_departure_route;

pub fn create_app(client: Arc<Client>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .nest("/", train_route(client.clone()))
        .nest("/", station_search_route(client.clone()))
        .nest("/", station_arrival_route(client.clone()))
        .nest("/", station_departure_route(client.clone()))
        .layer(cors)
}
