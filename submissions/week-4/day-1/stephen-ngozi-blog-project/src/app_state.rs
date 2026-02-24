use sqlx::PgPool;
use std::sync::Arc;



#[derive(Debug,Clone)]

// pub struct AppState {
//     pub db_pool: PgPool,
// }
pub struct  AppState {
    pub authors: Arc<RwLock<HashMap<uuid::Uuid,Author>>>,
    pub posts: Arc<RwLock<HashMap<uuid::Uuid, BlogPost>>>,
    // next_id: Arc<RwLock<u64>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            authors: Arc::new(RwLock::new(seed_authors())),
            posts: Arc::new(RwLock::new(HashMap::new())),
            // next_id: Arc::new(RwLock::new(0u64)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}