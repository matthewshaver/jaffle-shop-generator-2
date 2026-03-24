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

    let addr = "127.0.0.1:3000";
    println!("Jaffle Shop Generator running at http://localhost:3000");

    // Auto-open browser
    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        let _ = open::that("http://localhost:3000");
    });

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
