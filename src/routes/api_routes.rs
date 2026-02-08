use crate::handlers::api_handlers;
use crate::state::AppState;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn create_api_router() -> Router<AppState> {
    Router::new()
        .route("/api/books", get(api_handlers::list_books_api_handler))
        .route("/api/books", post(api_handlers::create_book_api_handler))
        .route(
            "/api/books/{id}",
            delete(api_handlers::delete_book_api_handler),
        )
        .route(
            "/api/books/{id}",
            put(api_handlers::update_book_api_handler),
        )
}
