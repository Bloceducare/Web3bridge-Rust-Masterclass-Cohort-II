use axum::{ Router,  routing::get};
use crate::handlers::{health_handler, get_posts,create_post,get_post,update_post,delete_post  };
use crate::models::{AppState};
use crate::root_handler;



pub fn create_app() -> Router {

    let state = AppState::new();

    Router::new()
    
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/posts", get(get_posts).post(create_post))
        .route("/posts/{id}", get(get_post).put(update_post).delete(delete_post),)
        .with_state(state)
}