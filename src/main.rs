use axum_test::TestServer;

use axum::{
    Router,
    extract::Path,
    routing::{delete, get, post, put},
};

async fn get_users() -> String {
    "List of users".to_string()
}

async fn create_user() -> String {
    "User created".to_string()
}

async fn update_user(Path(id): Path<u32>) -> String {
    format!("User {} updated", id)
}

async fn delete_user(Path(id): Path<u32>) -> String {
    format!("User {} deleted", id)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users/{id}", put(update_user)) // ← Pakai {id}
        .route("/users/{id}", delete(delete_user)); // ← Pakai {id}

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

#[tokio::test]
async fn test_all_routes() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user));

    let server = TestServer::new(app);

    // Test GET /
    let response = server.get("/").await;
    response.assert_status_ok();
    response.assert_text("Hello World");

    // Test GET /users
    let response = server.get("/users").await;
    response.assert_status_ok();
    response.assert_text("List of users");

    // Test POST /users
    let response = server.post("/users").await;
    response.assert_status_ok();
    response.assert_text("User created");

    // Test PUT /users/123
    let response = server.put("/users/123").await;
    response.assert_status_ok();
    response.assert_text("User 123 updated");

    // Test DELETE /users/456
    let response = server.delete("/users/456").await;
    response.assert_status_ok();
    response.assert_text("User 456 deleted");
}
