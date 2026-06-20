use axum::{Router, routing::post};

async fn echo(body: String) -> String {
    format!("You said: {}", body)
}

// curl -X POST http://localhost:3000/echo -d "Hello World"
// Output: You said: Hello World

#[tokio::main]
async fn main() {
    let app = Router::new().route("/echo", post(echo));
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
