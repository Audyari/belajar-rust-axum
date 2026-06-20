use axum::{
    Router,
    body::Body, // ← Import Body!
    http::{Request, StatusCode},
    middleware::from_fn,
    response::Response,
    routing::get,
};

async fn log_middleware(
    req: Request<Body>,           // ← 1️⃣ Request masuk
    next: axum::middleware::Next, // ← 2️⃣ Next = fungsi selanjutnya
) -> Result<Response, StatusCode> {
    // ← 3️⃣ Return Response atau Error

    println!("📥 {} {}", req.method(), req.uri().path());
    let response = next.run(req).await; // ← 3️⃣ Jalankan handler
    println!("📤 {}", response.status());
    println!("---");
    Ok(response)
}

async fn hello() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .layer(from_fn(log_middleware));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
