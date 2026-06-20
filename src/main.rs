use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

async fn handler() -> impl IntoResponse {
    (StatusCode::OK, "Hello World")
}

// curl http://localhost:3000/
// Output: Hello World

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
