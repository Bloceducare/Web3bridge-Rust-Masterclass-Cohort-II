
use uuid::Uuid;
use crate::models::{Author, AuthorList, AppState, BlogPost, CreatePostRequest, UpdatePostRequest};
use crate::error::AppError;
use axum::http::StatusCode;

pub async fn fetch_all_authors(state: &AppState) -> AuthorList {
    let authors = state.authors.read().await;
    let data: Vec<Author> = authors
        .iter()
        .map(|(_, a)| Author {
            id: a.id,
            name: a.name.clone(),
            email: a.email.clone(),
        })
        .collect();
    let total = data.len();
    AuthorList { data, total }
}

pub async fn fetch_author_by_id(state: &AppState, id: Uuid) -> Result<Author, AppError> {
    let authors = state.authors.read().await;
    authors
        .get(&id)
        .cloned()
        .ok_or(AppError::NotFound)
}


pub async fn insert_post (
    state: &AppState,
     payload: CreatePostRequest,) 
    -> Result<BlogPost, AppError>

{
    if payload.title.trim().is_empty() {
       return Err(AppError::UnProcessableEntity {
                        field: "title".to_string(),
                        message: "Title cannot be empty".to_string(),
                    
                    
        });
    }
    if payload.content.trim().is_empty() {
        return Err(AppError::UnProcessableEntity {
                        field: "content".to_string(),
                        message: "Content cannot be empty".to_string(),
                    
                    
                    
        });
    }
   

     let id = Uuid::new_v4();
    let now = chrono::Utc::now();
   

    let post =  BlogPost {
        id,
        title: payload.title,
        content: payload.content,
        author_id: payload.author_id,
        created_at: now,
        updated_at:now
    };

    let mut posts = state.posts.write().await;
    posts.insert(id, post.clone());

    Ok(post)


}

// pub async fn get_post (
//     State(state): State<AppState>,Path(id): Path<Uuid>,   
//  ) -> Result<Json<BlogPost>, AppError> {
//         let posts = state.posts.read().await;

//         match posts.get(&id) {
//             Some(post) => Ok(Json(post.clone())),
//             None => Err(AppError::NotFound),
            
//         }
// }

pub async fn fetch_post_by_id(state: &AppState, id: Uuid) -> Result<BlogPost, AppError> {
    let posts = state.posts.read().await;
    posts.get(&id).cloned().ok_or(AppError::NotFound)
}

// Get all Posts


pub async fn fetch_all_posts(state: &AppState, author_id: Option<Uuid>) -> Vec<BlogPost> {
    let posts = state.posts.read().await;
    match author_id {
        None => posts.values().cloned().collect(),
        Some(id) => posts
            .values()
            .filter(|p| p.author_id == id)
            .cloned()
            .collect(),
    }
}
// pub async fn fetch_all_posts(State(state): State<AppState>) -> impl IntoResponse{
//     let posts = state.posts.read().await ;
//     let post_list: Vec<BlogPost> = posts.values().cloned().collect();
//     Json(post_list)
// }



pub async fn modify_post ( 
     state: &AppState,
     id: Uuid,
     payload: UpdatePostRequest,
    )-> Result<BlogPost, AppError> {

    let mut posts = state.posts.write().await;

    let post = posts.get_mut(&id).ok_or(AppError::NotFound)?;

    if let Some(title) = payload.title {
        if title.trim().is_empty() {
            return Err(AppError::UnProcessableEntity {
                field: "title".into(),
                message: "Title cannot be empty".into(),
            });
        }
        post.title = title;
    }

    if let Some(content) = payload.content {
        if content.trim().is_empty() {
            return Err(AppError::UnProcessableEntity {
                field: "content".into(),
                message: "Content cannot be empty".into(),
            });
        }
        post.content = content;
    }

    if let Some(author_id) = payload.author_id {
        post.author_id = author_id;
    }

    post.updated_at = chrono::Utc::now();

    Ok(post.clone())

}


pub async fn remove_post ( 
        state: &AppState,
        id: Uuid,
    )-> Result<StatusCode, AppError> {

    let mut posts = state.posts.write().await;

    posts.remove(&id).map(|_| StatusCode::NO_CONTENT).ok_or(AppError::NotFound)

}
