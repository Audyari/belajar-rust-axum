use axum::{
    Router,
    extract::Multipart, // ← Sekarang bisa!
    routing::post,
};

async fn upload(mut multipart: Multipart) -> String {
    let mut file_names = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("unknown").to_string();
        file_names.push(name);
    }
    format!("Uploaded: {}", file_names.join(", "))
}

// echo Hello World > test.txt
// echo dummy > image.png

// curl -X POST http://localhost:3000/upload -F "file1=@test.txt" -F "file2=@image.png"
// Output: Uploaded: file1, file2

#[tokio::main]
async fn main() {
    let app = Router::new().route("/upload", post(upload));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
