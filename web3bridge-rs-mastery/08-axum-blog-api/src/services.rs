use crate::models::{BlogPost, CreatePostRequest, UpdatePostRequest};
use crate::state::AppState;
use std::collections::HashMap;

pub async fn get_all_posts(state: &AppState) -> Vec<BlogPost> {
    let posts = state.posts.read().await;
    posts.values().cloned().collect()
}

pub async fn get_post_by_id(state: &AppState, id: u64) -> Option<BlogPost> {
    let posts = state.posts.read().await;
    posts.get(&id).cloned()
}

pub async fn create_new_post(state: &AppState, payload: CreatePostRequest) -> BlogPost {
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
    
    post
}

pub async fn update_existing_post(
    state: &AppState, 
    id: u64, 
    payload: UpdatePostRequest
) -> Result<BlogPost, String> {
    let mut posts = state.posts.write().await;

    match posts.get_mut(&id) {
        Some(post) => {
            // Update fields if provided
            if let Some(title) = payload.title {
                if title.trim().is_empty() {
                    return Err("Title cannot be empty".to_string());
                }
                post.title = title;
            }

            if let Some(content) = payload.content {
                if content.trim().is_empty() {
                    return Err("Content cannot be empty".to_string());
                }
                post.content = content;
            }

            if let Some(author) = payload.author {
                if author.trim().is_empty() {
                    return Err("Author cannot be empty".to_string());
                }
                post.author = author;
            }

            post.updated_at = chrono::Utc::now();
            
            Ok(post.clone())
        }
        None => Err(format!("Post with id {} not found", id)),
    }
}

pub async fn delete_post_by_id(state: &AppState, id: u64) -> Result<(), String> {
    let mut posts = state.posts.write().await;
    match posts.remove(&id) {
        Some(_) => Ok(()),
        None => Err(format!("Post with id {} not found", id)),
    }
}
