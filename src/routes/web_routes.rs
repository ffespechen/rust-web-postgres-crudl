// Este módulo define las rutas para servir páginas HTML, como el dashboard.
use crate::handlers::web_handlers::{
    book_detail_web_handler, book_list_handler, create_book_web_handler,
    create_comment_web_handler, delete_book_web_handler, edit_book_web_handler, new_book_handler,
    update_book_web_handler,
};
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_web_router() -> Router<AppState> {
    Router::new()
        .route("/web/books", get(book_list_handler))
        .route("/web/books/new", get(new_book_handler))
        .route("/web/books", post(create_book_web_handler))
        .route("/web/books/delete/{id}", post(delete_book_web_handler))
        .route("/web/books/edit/{id}", get(edit_book_web_handler))
        .route("/web/books/update/{id}", post(update_book_web_handler))
        .route("/web/books/{id}", get(book_detail_web_handler))
        .route("/web/books/{id}/comments", post(create_comment_web_handler))
}
