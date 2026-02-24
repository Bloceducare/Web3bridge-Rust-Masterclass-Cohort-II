
use crate::services;
use crate::error::AppError;
use serde_json::json;
use crate::models::{AppState, AuthorList, BlogPost, CreatePostRequest, UpdatePostRequest};
use axum::{Json, extract::{Path,State}, http::StatusCode, response::{IntoResponse}};
use uuid::Uuid;

pub async  fn health_handler()-> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "message": "Server is running"
        })),
    )
    
}



pub async fn health() -> &'static str {
    "Hello, World!, app is working fine"
}


pub async fn get_authors(State(state): State<AppState>) -> Json<AuthorList> {
    Json(services::fetch_all_authors(&state).await)
}

pub async fn get_author_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let author = services::fetch_author_by_id(&state, id).await?;
    Ok((StatusCode::OK, Json(json!({ "data": author }))))
}


pub async fn get_posts(State(state): State<AppState>) -> impl IntoResponse {
    let posts = services::fetch_all_posts(&state, None).await;
    Json(posts)
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BlogPost>, AppError> {
    let post = services::fetch_post_by_id(&state, id).await?;
    Ok(Json(post))
}

pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<BlogPost>), AppError> {
    let post = services::insert_post(&state, payload).await?;
    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<BlogPost>, AppError> {
    let post = services::modify_post(&state, id, payload).await?;
    Ok(Json(post))
}

pub async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    services::remove_post(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}