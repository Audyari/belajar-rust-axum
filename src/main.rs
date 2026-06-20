use axum::{Router, extract::State, routing::get};
use std::sync::Arc;

struct AppState {
    counter: u32,
    message: String,
}

async fn home(State(state): State<Arc<AppState>>) -> String {
    format!("{} - Counter: {}", state.message, state.counter)
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        counter: 10,
        message: "Hello".to_string(),
    });

    let app = Router::new().route("/", get(home)).with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
