use axum::{
    Json,
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    // routing::{delete, get, patch, post, put},
    routing::get,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    net::{Ipv6Addr, SocketAddr},
    sync::Arc,
};
use tokio::{net::TcpListener, sync::RwLock};

// Assignment 1: Implement the Blog Post API with the following endpoints:
// - GET /posts: Retrieve a list of all blog posts.
// - GET /posts/{id}: Retrieve a specific blog post by its ID.
// - POST /posts: Create a new blog post.
// - PUT /posts/{id}: Update an existing blog post by its ID.
// - DELETE /posts/{id}: Delete a blog post by its ID.
// Use an in-memory data structure (e.g., a HashMap) to store the blog posts, and ensure that the API handles concurrent requests safely.
// Error handling should be implemented to return appropriate HTTP status codes and messages for different scenarios (e.g., post not found, invalid input, etc.).
// JSON should be used for request and response bodies, and the API should follow RESTful principles.
// Learn how to use Axum's routing, state management, and error handling features to build a robust and efficient web API in Rust.
// Learn Sequence Selection and Iteration.
// We should not manage everything in the handler we should break the handler down into services.
// Then we should have models. Models like Author, etc
// Learn how use the crate `secrecy` to handle environment variables
// Implement with sqlx

// Blog post structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BlogPost {
    id: u64,
    title: String,
    content: String,
    author: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    updated_at: chrono::DateTime<chrono::Utc>,
}

// Request body for creating a new post
#[derive(Debug, Deserialize)]
struct CreatePostRequest {
    title: String,
    content: String,
    author: String,
}

// Request body for updating a post
#[derive(Debug, Deserialize)]
struct UpdatePostRequest {
    title: Option<String>,
    content: Option<String>,
    author: Option<String>,
}

// Application state with thread-safe storage
#[derive(Clone)]
struct AppState {
    posts: Arc<RwLock<HashMap<u64, BlogPost>>>,
    next_id: Arc<RwLock<u64>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            posts: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(1)),
        }
    }
}

// Error response structure
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Health check response structure
#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

async fn root_handler() -> impl IntoResponse {
    let response = HealthResponse {
        status: "ok".to_string(),
        message: "Welcome to the Blog Post API".to_string(),
    };
    (StatusCode::OK, Json(response))
}

async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "message": "Server is running"
        })),
    )
}

// GET /posts - Retrieve all blog posts
async fn get_posts(State(state): State<AppState>) -> impl IntoResponse {
    let posts = state.posts.read().await;
    let post_list: Vec<BlogPost> = posts.values().cloned().collect();
    Json(post_list)
}

// GET /posts/{id} - Retrieve a specific blog post
async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<BlogPost>, (StatusCode, Json<ErrorResponse>)> {
    let posts = state.posts.read().await;

    match posts.get(&id) {
        Some(post) => Ok(Json(post.clone())),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Post with id {} not found", id),
            }),
        )),
    }
}

// POST /posts - Create a new blog post
async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<BlogPost>), (StatusCode, Json<ErrorResponse>)> {
    // Validate input
    if payload.title.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Title cannot be empty".to_string(),
            }),
        ));
    }

    if payload.content.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Content cannot be empty".to_string(),
            }),
        ));
    }

    if payload.author.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Author cannot be empty".to_string(),
            }),
        ));
    }

    // Generate new ID
    let mut next_id = state.next_id.write().await;
    let id = *next_id;
    *next_id += 1;
    drop(next_id);

    // Create new post
    let now = chrono::Utc::now();
    let post = BlogPost {
        id,
        title: payload.title,
        content: payload.content,
        author: payload.author,
        created_at: now,
        updated_at: now,
    };

    // Store the post
    let mut posts = state.posts.write().await;
    posts.insert(id, post.clone());

    Ok((StatusCode::CREATED, Json(post)))
}

// PUT /posts/{id} - Update an existing blog post
async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<BlogPost>, (StatusCode, Json<ErrorResponse>)> {
    // Validate at least one field is provided
    if payload.title.is_none() && payload.content.is_none() && payload.author.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "At least one field must be provided for update".to_string(),
            }),
        ));
    }

    let mut posts = state.posts.write().await;

    match posts.get_mut(&id) {
        Some(post) => {
            // Update fields if provided
            if let Some(title) = payload.title {
                if title.trim().is_empty() {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(ErrorResponse {
                            error: "Title cannot be empty".to_string(),
                        }),
                    ));
                }
                post.title = title;
            }

            if let Some(content) = payload.content {
                if content.trim().is_empty() {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(ErrorResponse {
                            error: "Content cannot be empty".to_string(),
                        }),
                    ));
                }
                post.content = content;
            }

            if let Some(author) = payload.author {
                if author.trim().is_empty() {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(ErrorResponse {
                            error: "Author cannot be empty".to_string(),
                        }),
                    ));
                }
                post.author = author;
            }

            post.updated_at = chrono::Utc::now();

            Ok(Json(post.clone()))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Post with id {} not found", id),
            }),
        )),
    }
}

// DELETE /posts/{id} - Delete a blog post
async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let mut posts = state.posts.write().await;

    match posts.remove(&id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Post with id {} not found", id),
            }),
        )),
    }
}

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
