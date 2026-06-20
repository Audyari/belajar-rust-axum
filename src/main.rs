use axum::{Router, routing::get};
use axum_test::TestServer;

#[tokio::main] // ← Baris 1: Macro untuk async main
async fn main() {
    // ← Baris 2: Fungsi utama async
    let app = Router::new().route("/", get(|| async { "Hello World" })); // ← Baris 3: Buat router & handler
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap(); // ← Baris 4: Binding port
    axum::serve(listener, app).await.unwrap(); // ← Baris 5: Jalankan server
}

#[tokio::test]
async fn test_hello() {
    // 1. Buat app
    let app = Router::new().route("/", get(|| async { "Hello World" }));

    // 2. Buat test server (tanpa unwrap!)
    let server = TestServer::new(app);

    // 3. Kirim request
    let response = server.get("/").await;

    // 4. Validasi response
    response.assert_status_ok();
    response.assert_text("Hello World");
}
