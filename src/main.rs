use axum::{Router, routing::get};

// ============================================
// HANDLERS
// ============================================
async fn home() -> &'static str {
    "Home"
}
async fn about() -> &'static str {
    "About"
}

// --- USERS ---
async fn users_list() -> &'static str {
    "Users List"
}
async fn users_detail() -> &'static str {
    "User Detail"
}

// --- PRODUCTS ---
async fn products_list() -> &'static str {
    "Products List"
}
async fn products_detail() -> &'static str {
    "Product Detail"
}

// --- ADMIN ---
async fn admin_dashboard() -> &'static str {
    "Admin Dashboard"
}
async fn admin_users() -> &'static str {
    "Admin Users"
}

// ============================================
// MAIN
// ============================================
#[tokio::main]
async fn main() {
    // ============================================
    // 1️⃣ ROUTER PUBLIK (Merge)
    // ============================================
    let router1 = Router::new().route("/", get(home));
    let router2 = Router::new().route("/about", get(about));

    // ============================================
    // 2️⃣ ROUTER USERS (Nest)
    // ============================================
    let users_router = Router::new()
        .route("/", get(users_list))
        .route("/{id}", get(users_detail));

    // ============================================
    // 3️⃣ ROUTER PRODUCTS (Nest)
    // ============================================
    let products_router = Router::new()
        .route("/", get(products_list))
        .route("/{id}", get(products_detail));

    // ============================================
    // 4️⃣ ROUTER ADMIN (Nest)
    // ============================================
    let admin_router = Router::new()
        .route("/", get(admin_dashboard))
        .route("/users", get(admin_users));

    // ============================================
    // 5️⃣ GABUNGIN SEMUA (Merge + Nest)
    // ============================================
    let app = Router::new()
        // Merge: Tanpa prefix
        .merge(router1) // → /
        .merge(router2) // → /about
        // Nest: Dengan prefix
        .nest("/users", users_router) // → /users, /users/123
        .nest("/products", products_router) // → /products, /products/456
        .nest("/admin", admin_router); // → /admin, /admin/users

    // ============================================
    // 6️⃣ RUN SERVER
    // ============================================
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    println!("\n📌 TESTING:");
    println!("  curl http://localhost:3000/");
    println!("  curl http://localhost:3000/about");
    println!("  curl http://localhost:3000/users");
    println!("  curl http://localhost:3000/users/123");
    println!("  curl http://localhost:3000/products");
    println!("  curl http://localhost:3000/products/456");
    println!("  curl http://localhost:3000/admin");
    println!("  curl http://localhost:3000/admin/users");

    axum::serve(listener, app).await.unwrap();
}
