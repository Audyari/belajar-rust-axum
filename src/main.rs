use axum::{
    Router,
    http::{Request, StatusCode},
    routing::get,
};

// ============================================
// 1️⃣ HANDLERS
// ============================================
async fn route_first() -> &'static str {
    "First Page"
}

async fn route_second() -> &'static str {
    "Second Page"
}

// ============================================
// 2️⃣ FALLBACK (404 Not Found)
// ============================================
async fn fallback(request: Request<axum::body::Body>) -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        format!("Page {} is not found", request.uri().path()),
    )
}

// ============================================
// 3️⃣ METHOD NOT ALLOWED FALLBACK (405)
// ============================================
async fn not_allowed(request: Request<axum::body::Body>) -> (StatusCode, String) {
    (
        StatusCode::METHOD_NOT_ALLOWED,
        format!("Method not allowed for {}", request.uri().path()),
    )
}

// ============================================
// 4️⃣ MAIN
// ============================================
#[tokio::main]
async fn main() {
    let first = Router::new().route("/first", get(route_first));
    let second = Router::new().route("/second", get(route_second));

    let app = Router::new()
        .merge(first)
        .merge(second)
        .fallback(fallback) // ← 404 Not Found
        .method_not_allowed_fallback(not_allowed); // ← 405 Method Not Allowed

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    println!("\n📌 Route yang tersedia (hanya GET):");
    println!("  GET /first  → First Page");
    println!("  GET /second → Second Page");
    println!("\n📌 Coba method yang salah:");
    println!("  curl -X POST http://localhost:3000/first");
    println!("  curl -X PUT http://localhost:3000/second");
    println!("\n📌 Coba route yang tidak ada:");
    println!("  curl http://localhost:3000/wrong");
    axum::serve(listener, app).await.unwrap();
}
