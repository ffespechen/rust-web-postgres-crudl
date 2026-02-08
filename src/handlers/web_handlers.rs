use crate::models::models::{Book, CreateBook};
use crate::state::AppState;
use askama::Template;
use axum::response::Redirect;
use axum::{
    Form,
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::Html,
};
use serde::Deserialize;
use sqlx::{self};
use std::fs;

// Tamaño máximo permitido para la imagen de portada (2 MB)
const MAX_IMAGE_SIZE: usize = 2 * 1024 * 1024;

// Página de inicio que muestra el número total de libros en la base de datos
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub count: i64,
}

// Función genérica para validar tamaño y tipo de imagen
async fn save_image(
    field: axum::extract::multipart::Field<'_>,
) -> Result<String, (StatusCode, String)> {
    let content_type = field
        .content_type()
        .map(|ct| ct.to_string())
        .unwrap_or_default();

    // 1. Validar el tipo de contenido
    if !content_type.starts_with("image/") {
        return Err((StatusCode::BAD_REQUEST, "File must be an image".to_string()));
    }

    let data = field
        .bytes()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 2. Validar el tamaño del archivo
    if data.len() > MAX_IMAGE_SIZE {
        return Err((
            StatusCode::BAD_REQUEST,
            "Image size exceeds 2 MB".to_string(),
        ));
    }

    // 3. Guardar la imagen en el sistema de archivos
    if !data.is_empty() {
        let filename = format!("uploads/{}.jpg", uuid::Uuid::new_v4());
        fs::write(&filename, &data)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        return Ok(filename);
    }

    Ok(String::from("uploads/portada_generica.jpg"))
}

// Función para manejar la ruta raíz y mostrar el número de libros
pub async fn index_handler(
    State(state): State<AppState>,
) -> Result<Html<String>, (StatusCode, String)> {
    // Contar el número de libros en la base de datos
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*)::BIGINT FROM books")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Renderizar la plantilla con el conteo de libros
    let html = IndexTemplate { count }
        .render()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Html(html))
}

// Estructura para manejar los parámetros de búsqueda en la ruta de listado de libros

#[derive(Deserialize)]
pub struct SearchParams {
    pub search: Option<String>,
}
// [L] Estructura para la plantilla de página de listado de libros
#[derive(Template)]
#[template(path = "book_list.html")]
pub struct BookListTemplate {
    pub books: Vec<Book>,
}

// [L] Función para manejar la ruta de listado de libros
pub async fn book_list_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Html<String>, (StatusCode, String)> {
    // Lóica de listado sin filtros
    // let books = sqlx::query_as!(Book, "SELECT id, title, author, published_date FROM books")
    //     .fetch_all(&state.db_pool)
    //     .await
    //     .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let books = match params.search {
        Some(ref query) if !query.trim().is_empty() => {
            let like_query = format!("%{}%", query);
            sqlx::query_as::<_, Book>("SELECT * FROM books WHERE title ILIKE $1 OR author ILIKE $1")
                .bind(like_query)
                .fetch_all(&state.db_pool)
                .await
        }
        _ => {
            sqlx::query_as::<_, Book>("SELECT * FROM books")
                .fetch_all(&state.db_pool)
                .await
        }
    }
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let html = BookListTemplate { books }
        .render()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Html(html))
}

// [C] Estructura para la plantilla de página de crear un libro
#[derive(Template)]
#[template(path = "book_create.html")]
pub struct BookCreateTemplate;

pub async fn new_book_handler() -> Result<Html<String>, (StatusCode, String)> {
    let html = BookCreateTemplate
        .render()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Html(html))
}

// [C] Función para manejar la creación de un nuevo libro
// En esta primera versión no se maneja la subida de imágenes
// pub async fn create_book_web_handler(
//     State(state): State<AppState>,
//     Form(payload): Form<CreateBook>,
// ) -> Result<Redirect, (StatusCode, String)> {
//     let new_id = uuid::Uuid::new_v4();
//     let _ = sqlx::query_as::<_, Book>(
//         "INSERT INTO books (id, title, author, published_date) VALUES ($1, $2, $3, $4) RETURNING id, title, author, published_date",

//     )
//     .bind(new_id)
//     .bind(&payload.title)
//     .bind(&payload.author)
//     .bind(payload.published_date)
//     .fetch_one(&state.db_pool)
//     .await
//     .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

//     Ok(Redirect::to("/web/books"))
// }

// [C] Función para manejar la creación de un nuevo libro con subida de imagen
pub async fn create_book_web_handler(
    State(state): State<AppState>,
    mut multipart: axum::extract::Multipart,
) -> Result<Redirect, (StatusCode, String)> {
    let mut title = String::new();
    let mut author = String::new();
    let mut published_date: i32 = 0;
    let mut image_path: String = String::from("uploads/portada_generica.jpg");

    // Iteramos sobre los campos del formulario multipart
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        match name.as_str() {
            "title" => {
                title = field.text().await.unwrap();
            }
            "author" => {
                author = field.text().await.unwrap();
            }
            "published_date" => {
                published_date = field.text().await.unwrap().parse::<i32>().unwrap();
            }
            "cover" => {
                // Solo intentamos guardar si el usuario subió algo
                if let Ok(path) = save_image(field).await {
                    if path != "uploads/default.jpg" {
                        image_path = path;
                    }
                }
            }
            _ => {}
        }
    }

    let new_id = uuid::Uuid::new_v4();
    let _ = sqlx::query(
        "INSERT INTO books (id, title, author, published_date, image_path) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(new_id)
    .bind(&title)
    .bind(&author)
    .bind(published_date)
    .bind(&image_path)
    .execute(&state.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Redirect::to("/web/books"))
}

pub async fn delete_book_web_handler(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Redirect, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM books WHERE id = $1")
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Book not found".to_string()));
    }

    Ok(Redirect::to("/web/books"))
}

// -- [U] Función para manejar la edición de un libro

#[derive(Template)]
#[template(path = "book_edit.html")]
pub struct BookEditTemplate {
    pub book: Book,
}

pub async fn edit_book_web_handler(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Html<String>, (StatusCode, String)> {
    let book = sqlx::query_as::<_, Book>(
        "SELECT id, title, author, published_date FROM books WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let html = BookEditTemplate { book }
        .render()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Html(html))
}

pub async fn update_book_web_handler(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Form(payload): Form<CreateBook>,
) -> Result<Redirect, (StatusCode, String)> {
    let _ =
        sqlx::query("UPDATE books SET title = $1, author = $2, published_date = $3 WHERE id = $4")
            .bind(&payload.title)
            .bind(&payload.author)
            .bind(payload.published_date)
            .bind(id)
            .execute(&state.db_pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Redirect::to("/web/books"))
}
