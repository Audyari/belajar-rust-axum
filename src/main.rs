use axum::{Router, extract::Query, routing::get};
use serde::Deserialize;

#[derive(Deserialize)]
struct Filter {
    page: u32,
    limit: u32,
}

async fn list_items(Query(filter): Query<Filter>) -> String {
    format!("Page: {}, Limit: {}", filter.page, filter.limit)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/items", get(list_items));
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
