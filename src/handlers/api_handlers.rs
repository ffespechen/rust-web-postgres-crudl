use crate::models::models::{Book, Comment, CreateBook, CreateComment};
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

// [L] Manejador API para listar todos los libros
pub async fn list_books_api_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Book>>, StatusCode> {
    let books = sqlx::query_as::<_, Book>("SELECT id, title, author, published_date FROM books")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(books))
}

pub async fn create_book_api_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateBook>,
) -> Result<(StatusCode, Json<Book>), (StatusCode, String)> {
    let new_id = Uuid::new_v4();
    let new_book = sqlx::query_as::<_, Book>(
        "INSERT INTO books (id, title, author, published_date) VALUES ($1, $2, $3, $4) RETURNING id, title, author, published_date",
    )
    .bind(new_id)
    .bind(&payload.title)
    .bind(&payload.author)
    .bind(payload.published_date)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(new_book)))
}

pub async fn delete_book_api_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM books WHERE id = $1")
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Book not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_book_api_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateBook>,
) -> Result<Json<Book>, (StatusCode, String)> {
    let updated_book = sqlx::query_as::<_, Book>(
        "UPDATE books SET title = $1, author = $2, published_date = $3 WHERE id = $4 RETURNING id, title, author, published_date",
    )
    .bind(&payload.title)
    .bind(&payload.author)
    .bind(payload.published_date)
    .bind(id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated_book))
}

// [C] Creación de un comentario para un libro específico
pub async fn create_comment_api_handler(
    State(state): State<AppState>,
    Path(book_id): Path<Uuid>,
    Json(payload): Json<CreateComment>,
) -> Result<(StatusCode, Json<Comment>), (StatusCode, String)> {
    let new_id = Uuid::new_v4();
    let new_comment = sqlx::query_as::<_, Comment>(
        "INSERT INTO comments (id, book_id, text) VALUES ($1, $2, $3) RETURNING id, book_id, text, created_at",
    )
    .bind(new_id)
    .bind(book_id)
    .bind(&payload.text)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(new_comment)))
}
