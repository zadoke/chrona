use axum::Router;
use reqwest::Client;
use std::sync::Arc;

use crate::routes::train::train_route;
use crate::routes::station::station_search_route;
use crate::routes::schedule::station_arrival_route;
use crate::routes::schedule::station_departure_route;

pub fn create_app(client: Arc<Client>) -> Router {
    Router::new()
        .nest("/", train_route(client.clone()))
        .nest("/", station_search_route(client.clone()))
        .nest("/", station_arrival_route(client.clone()))
        .nest("/", station_departure_route(client.clone()))
}
