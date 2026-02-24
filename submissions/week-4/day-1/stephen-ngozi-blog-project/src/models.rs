use std::{collections::HashMap,sync::Arc};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use ::serde::{Deserialize, Serialize};
use tokio:: sync::RwLock;
// use axum::{Json, Router, extract::{Path,State}, http::StatusCode, response::{IntoResponse}, routing::get};



// Author 
#[derive(Serialize, Deserialize, Clone)]
pub struct Author {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl Author {
    pub fn new(id: Uuid, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}



#[derive(Serialize)]
pub struct AuthorList {
    pub data: Vec<Author>,
    pub total: usize,
}

// BlogPost 
#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct BlogPost  {
    pub id:Uuid,
    pub title: String,
    pub content: String,
    pub author_id: uuid::Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>
}

// Request DTOs

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
   
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    
}
#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
   
    pub title: Option<String>,
    pub content: Option<String>,
    pub author_id: Option<Uuid>,
  
}

// AppState 



// pub async fn get_author_by_id(&self, author_id: u16) -> Option<Author> {
//     let authors = self.authors.read().await;
//     authors.iter().find(|a| a.id == author_id).cloned()
// }





fn seed_authors() -> HashMap<Uuid, Author> {
    let authors = vec![
        Author::new(
            Uuid::new_v4(),
            "Emeka Allison".to_string(),
            "emekaallison4@gmail.com".to_string(),
        ),
        Author::new(
            Uuid::new_v4(),
            "John Doe".to_string(),
            "johndoe@mail.com".to_string(),
        ),
    ];

    authors.into_iter().map(|a| (a.id, a)).collect()
}


// pub async fn find_posts(&self, author_id: Option<u16>) -> Vec<Post> {
//         let posts = self.posts.read().await;
//         match author_id {
//             None => posts.clone(),
//             Some(id) => posts
//                 .iter()
//                 .filter(|p| p.author_id == id)
//                 .cloned()
//                 .collect(),
//         }
//     }
// }