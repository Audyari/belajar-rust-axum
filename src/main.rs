use axum::{Json, Router, extract::rejection::JsonRejection, routing::post};
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
struct LoginRequest {
    username: String,
    password: String,
}

// ✅ HANDLE JSON REJECTION DENGAN Result
async fn login(payload: Result<Json<LoginRequest>, JsonRejection>) -> String {
    match payload {
        Ok(request) => {
            format!("Hello {}", request.username)
        }
        Err(error) => {
            format!("Error: {:?}", error)
        }
    }
}

// curl -X POST http://localhost:3000/login -H "Content-Type: application/json" -d "{\"username\":\"admin\",\"password\":\"12345\"}"
// Output: Hello admin

// curl -X POST http://localhost:3000/login -H "Content-Type: application/json" -d "{\"username\":\"admin\"}"
// Output: Error: InvalidJsonBody: missing field `password` at line 1 column 20

#[tokio::main]
async fn main() {
    let app = Router::new().route("/login", post(login));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
