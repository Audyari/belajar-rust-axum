use axum::{Router, extract::Query, routing::get};
use http::HeaderMap;
use serde::Deserialize;

#[derive(Deserialize)]
struct Filter {
    page: u32,
    limit: u32,
}

async fn list_items(Query(filter): Query<Filter>) -> String {
    format!("Page: {}, Limit: {}", filter.page, filter.limit)
}

async fn read_headers(headers: HeaderMap) -> String {
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Unknown");
    format!("User-Agent: {}", user_agent)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/headers", get(read_headers))
        .route("/items", get(list_items));
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
