mod db;
mod handlers;
mod models;
mod routes;
mod state;

use crate::handlers::web_handlers;
use crate::routes::api_routes::create_api_router;
use axum::{Router, routing::get};
use dotenvy::dotenv;
use routes::web_routes::create_web_router;

use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 1. Cargar .env
    dotenv().ok();

    // 2. Configurar la conexión a la base de datos
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = db::init_db_pool(&database_url).await;

    // 4. Configurar el estado compartido
    let app_state = state::AppState { db_pool };

    // 5. Configurar las rutas
    let app = Router::new()
        .nest_service("/uploads", tower_http::services::ServeDir::new("./uploads"))
        .route("/", get(web_handlers::index_handler))
        .merge(create_web_router())
        .merge(create_api_router())
        .with_state(app_state);

    // 6. Iniciar el servidor
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Servidor escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("💥 Error al enlazar a la dirección");
    axum::serve(listener, app)
        .await
        .expect("💥 Error al iniciar el servidor");
}
