use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub published_date: i32,
    pub image_path: String,
}

#[derive(serde::Deserialize)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub published_date: i32,
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Comment {
    pub id: Uuid,
    pub book_id: Uuid,
    pub text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Deserialize)]
pub struct CreateComment {
    pub text: String,
}
