use axum::{Json, Router, routing::get};
use serde_json::json;

async fn user() -> Json<serde_json::Value> {
    Json(json!({
        "id": 1,
        "name": "Alice",
        "age": 30
    }))
}

// curl http://localhost:3000/user
// Output: {"id":1,"name":"Alice","age":30}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/user", get(user));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
