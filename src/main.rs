use axum::{Router, extract::State, routing::get};
use std::sync::Arc;

struct AppState {
    message: String,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        message: "Hello from closure!".to_string(),
    });

    // ✅ PAKAI STATE (paling aman untuk Axum)
    let app = Router::new()
        .route(
            "/",
            get(|State(state): State<Arc<AppState>>| async move { state.message.clone() }),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
