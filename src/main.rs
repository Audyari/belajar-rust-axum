use axum::{Router, routing::get};

async fn home() -> &'static str {
    "Home"
}
async fn about() -> &'static str {
    "About"
}

// curl http://localhost:3000/
// curl http://localhost:3000/about

#[tokio::main]
async fn main() {
    let router1 = Router::new().route("/", get(home));

    let router2 = Router::new().route("/about", get(about));

    // ✅ MERGE: Gabungin tanpa prefix
    let app = Router::new().merge(router1).merge(router2);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
