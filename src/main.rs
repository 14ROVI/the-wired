mod routes;
mod structs;
mod templates;
mod utils;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(templates::index))
        .route("/timeline", get(routes::get_timeline));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
