use axum::{
    Router,
    http::{Request, StatusCode},
    routing::get,
};

// ============================================
// 1️⃣ HANDLERS UNTUK ROUTE YANG ADA
// ============================================
async fn route_first() -> &'static str {
    "First Page"
}

async fn route_second() -> &'static str {
    "Second Page"
}

// ============================================
// 2️⃣ FALLBACK (404 Handler)
// ============================================
async fn fallback(request: Request<axum::body::Body>) -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        format!("Page {} is not found", request.uri().path()),
    )
}

// ============================================
// 4️⃣ MAIN
// ============================================
#[tokio::main]
async fn main() {
    // Route yang ada
    let first = Router::new().route("/first", get(route_first));
    let second = Router::new().route("/second", get(route_second));

    // Gabungin semua + Fallback
    let app = Router::new().merge(first).merge(second).fallback(fallback); // ← Ganti fallback_json untuk JSON response

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    println!("\n📌 Route yang tersedia:");
    println!("  GET /first  → First Page");
    println!("  GET /second → Second Page");
    println!("\n📌 Coba route yang tidak ada:");
    println!("  curl http://localhost:3000/wrong");
    println!("  curl http://localhost:3000/anything");
    axum::serve(listener, app).await.unwrap();
}
