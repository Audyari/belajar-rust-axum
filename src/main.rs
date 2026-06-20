use axum::{Form, Router, routing::post};
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)] // Tambahkan attribute ini agar field password tidak dianggap unused
struct LoginForm {
    username: String,
    password: String,
}

async fn login(Form(form): Form<LoginForm>) -> String {
    format!("Welcome {}", form.username)
}

// curl -X POST http://localhost:3000/login -H "Content-Type: application/x-www-form-urlencoded" -d "username=admin&password=password"
// Output: Welcome admin

#[tokio::main]
async fn main() {
    let app = Router::new().route("/login", post(login));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
