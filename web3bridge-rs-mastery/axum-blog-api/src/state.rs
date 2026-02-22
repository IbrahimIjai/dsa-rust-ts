use crate::models::BlogPost;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub posts: Arc<RwLock<HashMap<u64, BlogPost>>>,
    pub next_id: Arc<RwLock<u64>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            posts: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(1)),
        }
    }
}
