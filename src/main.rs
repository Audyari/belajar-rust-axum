use axum::{Router, extract::Query, response::IntoResponse, routing::get};
use std::collections::HashMap;

use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

// ============================================
// ROUTE: /query?name=Eko
// ============================================
async fn route(
    cookies: Cookies,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let name = query.get("name").unwrap();
    cookies.add(Cookie::new("name", name.clone()));
    format!("Hello {}", name)
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
