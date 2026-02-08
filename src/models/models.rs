use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub published_date: i32,
}

#[derive(serde::Deserialize)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub published_date: i32,
}
