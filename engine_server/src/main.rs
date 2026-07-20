use axum::{Router, routing::post};
mod handlers;
mod logic;

#[tokio::main]
async fn main() {
    println!("Server starting...");

    let app = Router::new()
        // Accessing: mod handlers -> mod handler -> fn handle_guess
        .route("/guess", post(handlers::handler::handle_guess))
        .route("/start", post(handlers::handler::handle_start_game));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
