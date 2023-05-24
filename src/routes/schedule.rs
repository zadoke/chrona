use axum::{extract::Path , routing::get , Router};
use reqwest::Client;
use std::sync::Arc;

use crate::handlers::schedule_handler::handler;

pub fn station_departure_route(client: Arc<Client>) -> Router {
    Router::new().route(
        "/station/:station_number/departures",
        get({
            move |path: Path<i32>| handler(client.clone(), path, 0) // Schedule type of 0 is departures, 1 is arrivals
        }),
    )
}


pub fn station_arrival_route(client: Arc<Client>) -> Router {
    Router::new().route(
        "/station/:station_number/arrivals",
        get({
            move |path: Path<i32>| handler(client.clone(), path, 1)
        }),
    )
}
