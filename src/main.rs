use axum::{Router, routing::get};

#[tokio::main] // ← Baris 1: Macro untuk async main
async fn main() {
    // ← Baris 2: Fungsi utama async
    let app = Router::new().route("/", get(|| async { "Hello World" })); // ← Baris 3: Buat router & handler
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap(); // ← Baris 4: Binding port
    axum::serve(listener, app).await.unwrap(); // ← Baris 5: Jalankan server
}
