use axum::{
    Router,
    extract::Path, // ← Tambahkan ini!
    routing::get,
};

async fn get_category(Path((product_id, category_id)): Path<(u32, String)>) -> String {
    format!("Product {} Category {}", product_id, category_id)
}

// panggil gunakan :  http://localhost:3000/products/123/categories/456

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/products/{product_id}/categories/{category_id}",
        get(get_category),
    );
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
