use axum::{Router, extract::Query, response::IntoResponse, routing::get};
use std::collections::HashMap;

use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

// ============================================
// ROUTE: /query?name=Eko
// ============================================

// ============================================
// 📨 REQUEST + 📤 RESPONSE
// ============================================
async fn route(
    // ============================================
    // 📨 INI REQUEST (Data dari client)
    // ============================================
    cookies: Cookies, // ← Request: baca cookie dari client
    Query(query): Query<HashMap<String, String>>, // ← Request: baca query dari client
) -> impl IntoResponse {
    // Proses data dari request
    let name = query.get("name").unwrap();

    // ============================================
    // 📤 INI RESPONSE (Data ke client)
    // ============================================
    cookies.add(Cookie::new("name", name.clone())); // ← Response: kirim cookie
    format!("Hello {}", name) // ← Response: kirim body
}

// ============================================
// MAIN
// ============================================

// http://localhost:3000/query?name=Audy
// curl "http://localhost:3000/query?name=Eko"
// curl -i "http://localhost:3000/query?name=Eko"
// curl -c cookies.txt "http://localhost:3000/query?name=Eko"
// curl -H "Cookie: name=Eko" "http://localhost:3000/query?name=eko"
// curl "http://localhost:3000/query?name=Alice"

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/query", get(route))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
