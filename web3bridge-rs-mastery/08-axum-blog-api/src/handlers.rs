use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::models::{BlogPost, CreatePostRequest, UpdatePostRequest, ErrorResponse, HealthResponse};
use crate::state::AppState;
use crate::services; // Importing the services module

pub async fn root_handler() -> impl IntoResponse {
    let response = HealthResponse {
        status: "ok".to_string(),
        message: "Welcome to the Blog Post API".to_string(),
    };
    (StatusCode::OK, Json(response))
}

pub async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "message": "Server is running"
        })),
    )
}

// GET /posts - Retrieve all blog posts
pub async fn get_posts(State(state): State<AppState>) -> impl IntoResponse {
    let post_list = services::get_all_posts(&state).await;
    Json(post_list)
}

// GET /posts/{id} - Retrieve a specific blog post
pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<BlogPost>, (StatusCode, Json<ErrorResponse>)> {
    match services::get_post_by_id(&state, id).await {
        Some(post) => Ok(Json(post)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Post with id {} not found", id),
            }),
        )),
    }
}

// POST /posts - Create a new blog post
pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<BlogPost>), (StatusCode, Json<ErrorResponse>)> {
    // Validate input logic (Should probably be in service or DTO validation, but keeping basic checks here or deferring)
    // The main.rs logic had validation inline. 
    // I will keep validation here as "handler logic" or move to service. 
    // Let's keep it here for now to respect "handler layer" handling inputs.
    
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

    let post = services::create_new_post(&state, payload).await;
    Ok((StatusCode::CREATED, Json(post)))
}

// PUT /posts/{id} - Update an existing blog post
pub async fn update_post(
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
    
    // Note: The original main.rs checked for empty strings inside the update logic.
    // My service implementation does check for empty strings and returns Result<BlogPost, String>.
    
    // I'll call the service logic which handles both the "not found" and the "validation error" (empty string)
    // However, the `update_existing_post` service I wrote returns simple String error. 
    // I need to map it to correct status code.
    
    match services::update_existing_post(&state, id, payload).await {
        Ok(post) => Ok(Json(post)),
        Err(err_msg) => {
            // Primitive error matching based on string content to decide 400 vs 404
            let status = if err_msg.contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::BAD_REQUEST
            };
            
            Err((
                status,
                Json(ErrorResponse {
                    error: err_msg,
                }),
            ))
        }
    }
}

// DELETE /posts/{id} - Delete a blog post
pub async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    match services::delete_post_by_id(&state, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err_msg) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: err_msg,
            }),
        )),
    }
}
