mod db;
mod routes;
mod structs;
mod templates;
mod utils;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let database_url =
        env::var("DATABASE_URL").expect("can't find DATABASE_URL environment variable");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Could not connect to postgres");

    // sqlx::migrate!("./migrations")
    //     .run(&pool)
    //     .await
    //     .expect("Could not migrate");

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/timeline", get(routes::get_timeline))
        .nest_service("/assets/", ServeDir::new("./assets/")) // in case of no nginx or similar
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
