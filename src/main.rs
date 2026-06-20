use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
};
use serde_json::json;
use thiserror::Error;

// ============================================
// 1️⃣ APPERROR
// ============================================
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

// ============================================
// 2️⃣ HANDLER DENGAN APPERROR
// ============================================
async fn get_user() -> Result<Json<serde_json::Value>, AppError> {
    let user_id = 1;

    if user_id == 0 {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    if user_id == 1 {
        return Ok(Json(json!({
            "id": 1,
            "name": "Alice",
            "email": "alice@example.com"
        })));
    }

    Err(AppError::Internal)
}

async fn create_user() -> Result<String, AppError> {
    let name = "".to_string();

    if name.is_empty() {
        return Err(AppError::Validation("Name is required".to_string()));
    }

    Ok("User created".to_string())
}

async fn admin_only() -> Result<String, AppError> {
    let token = "secret";

    if token != "secret" {
        return Err(AppError::Unauthorized);
    }

    Ok("Welcome admin".to_string())
}

// ============================================
// 3️⃣ MAIN
// ============================================

// 1. SUCCESS (GET /user)
// curl http://localhost:3000/user
// Output: {"id":1,"name":"Alice","email":"alice@example.com"}

// 2. VALIDATION ERROR (GET /create)
// curl http://localhost:3000/create
// Output: {"error":"Name is required","status":400}

// 3. UNAUTHORIZED (GET /admin)
// curl http://localhost:3000/admin
// Output: {"error":"Unauthorized","status":401}

// 4. NOT FOUND (ubah user_id = 0 di kode)
// curl http://localhost:3000/user
// Output: {"error":"User not found","status":404}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/user", get(get_user))
        .route("/create", get(create_user))
        .route("/admin", get(admin_only));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
