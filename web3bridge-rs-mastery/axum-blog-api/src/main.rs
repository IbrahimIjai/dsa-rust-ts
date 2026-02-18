use std::{
    any::Any,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
};

use axum::{Router, routing::get};
use tokio::net::TcpListener;

async fn health() -> &'static str {
    "Hello, World!, app is working fine"
}

enum BlogCategory {
    Tech,
    Lifestyle,
    Travel,
    Food,
    Other(String),
}
struct Blog {
    blog_id: String,
    category: BlogCategory,
    content: String,
    Likes: u32,
    Comments: Vec<String>,
    authors: Vec<String>,
}

impl Blog {
    fn new(blog_id: String, category: BlogCategory, content: String) -> Self {
        Self {
            blog_id,
            category,
            content,
            Likes: 0,
            Comments: Vec::new(),
            authors: Vec::new(),
        }
    }

    fn add_like(&mut self) {
        self.Likes += 1;
    }

    fn add_comment(&mut self, comment: String) {
        self.Comments.push(comment);
    }

    fn add_author(&mut self, author: String) {
        self.authors.push(author);
    }
}

fn create_app() -> Router {
    Router::new().route("/health", get(health))
}

fn get_env_vars<T>(key: String) -> Result<T, String>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let value = std::env::var(key)
        .map_err(|err| format!("Failed to read environment variable: {}", err))?;
    value.parse::<T>().map_err(|_| "".to_string())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let app: Router = create_app();
    let listening_port: u16 = 8080;

    let port: u16 = get_env_vars::<u16>("PORT".to_string()).unwrap_or(listening_port);

    let listening_address: SocketAddr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));

    let binder: TcpListener = TcpListener::bind(listening_address)
        .await
        .expect("Failed to bind to address  ");

    println!("Server is listening on {}", binder.local_addr().unwrap());

    axum::serve(binder, app).await.unwrap();
}

// Sequence, control, iteration
// ARC - Atomic 
// Rc - Reference Counting

// Impl with Secrecy, 
// Model with SQLS 