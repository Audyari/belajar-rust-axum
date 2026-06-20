use axum::{Router, extract::Json, routing::post};
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    name: String,
    age: u8,
}

async fn create_user(Json(payload): Json<User>) -> String {
    format!("User: {}, Age: {}", payload.name, payload.age)
}

// curl -X POST http://localhost:3000/user -H "Content-Type: application/json" -d "{\"name\":\"Alice\",\"age\":30}"
// Output: User: Alice, Age: 30

#[tokio::main]
async fn main() {
    let app = Router::new().route("/user", post(create_user));
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
