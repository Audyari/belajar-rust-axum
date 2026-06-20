use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    name: String,
    age: u8,
}

async fn create_user(Json(payload): Json<User>) -> String {
    format!("{} is {} years old", payload.name, payload.age)
}

async fn hello() -> String {
    "Hello World".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/user", post(create_user))
        .route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
