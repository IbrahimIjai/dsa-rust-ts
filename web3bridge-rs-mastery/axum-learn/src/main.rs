use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/", get(|| async { "Hello, World!" }));
    println!("Hello, world!");
}
