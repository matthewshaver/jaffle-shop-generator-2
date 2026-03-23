mod models;
mod simulation;
mod web;

use axum::{routing::{get, post}, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(web::handlers::index))
        .route("/api/generate", post(web::handlers::generate))
        .route("/api/defaults", get(web::handlers::default_config));

    let addr = "0.0.0.0:3000";
    println!("Jaffle Shop Generator running at http://localhost:3000");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
