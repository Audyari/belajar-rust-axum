use axum::{
    Router,
    body::Body,
    http::{HeaderValue, Request, StatusCode},
    middleware::from_fn,
    response::Response,
    routing::get,
};
use uuid::Uuid;

// ============================================
// 🔍 REQUEST ID MIDDLEWARE
// ============================================
async fn request_id_middleware(
    req: Request<Body>,
    next: axum::middleware::Next,
) -> Result<Response, StatusCode> {
    // 1️⃣ Baca atau buat Request ID
    let request_id = req
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    println!("📨 Request ID: {}", request_id);

    // 2️⃣ Lanjut ke handler
    let mut response = next.run(req).await;

    // 3️⃣ Tambahkan Request ID ke response
    response
        .headers_mut()
        .insert("x-request-id", HeaderValue::from_str(&request_id).unwrap());

    println!("📤 Response sent (ID: {})", request_id);
    Ok(response)
}

// ============================================
// HANDLER
// ============================================
async fn hello() -> &'static str {
    "Hello World"
}

// ============================================
// MAIN
// ============================================

// curl -i -H "x-request-id: 999" http://localhost:3000/

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .layer(from_fn(request_id_middleware));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
