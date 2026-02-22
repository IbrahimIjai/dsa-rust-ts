use axum::{
    Router,
    routing::get,
};
use std::net::{Ipv6Addr, SocketAddr};
use tokio::net::TcpListener;

mod models;
mod state;
mod handlers;
mod services;

use state::AppState;
use handlers::{root_handler, health_handler, get_posts, create_post, get_post, update_post, delete_post};

fn create_app() -> Router {
    let state = AppState::new();

    Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/posts", get(get_posts).post(create_post))
        .route(
            "/posts/{id}",
            get(get_post).put(update_post).delete(delete_post),
        )
        .with_state(state)
}

fn get_env_vars<T>(key: &str) -> Result<T, String>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let value = std::env::var(key)
        .map_err(|err| format!("Failed to read environment variable: {}", err))?;
    value
        .parse::<T>()
        .map_err(|e| format!("Failed to parse env var: {:?}", e))
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let app: Router = create_app();

    let port: u16 = get_env_vars::<u16>("PORT").unwrap_or(8080);

    let listening_address: SocketAddr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));

    let binder: TcpListener = TcpListener::bind(listening_address)
        .await
        .expect("Failed to bind to address  ");

    println!("Server is listening on {}", binder.local_addr().unwrap());

    axum::serve(binder, app).await.unwrap();
}
