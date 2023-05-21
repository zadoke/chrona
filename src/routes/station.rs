use axum::{extract::Path , routing::get , Router};
use reqwest::Client;
use std::sync::Arc;

use crate::handlers::station_handler::handler;

pub fn station_search_route(client: Arc<Client>) -> Router {
    Router::new().route(
        "/station/search/:query",
        get({
            move |path: Path<String>| handler(client.clone(), path)
        }),
    )
}
