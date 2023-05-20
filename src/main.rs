use axum::{extract::Path , response::IntoResponse, routing::get , Json , Router};
use crate::train::fetch_train_data;
use serde_json::json;

mod train;
mod utils;

async fn train_handler(Path(train_number): Path<i32>) -> impl IntoResponse {
    match fetch_train_data(&train_number).await {
        Ok(train_data) => Json(train_data),
        Err(e) => {
            eprintln!("Error fetching train data: {:?}", e);
            let error_response = json!({
                "error": "Error fetching train data",
                "details": format!("{:?}", e)
            });
            Json(error_response)
        }
    }
}
#[tokio::main]
async fn main() {
    let app = Router::new().route("/train/:train_number", get(train_handler));
    println!("CPTrainBot backend started sucessfully!");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
