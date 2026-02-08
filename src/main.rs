mod handlers;
mod models;
mod routes;
mod state;

use crate::handlers::web_handlers;
use crate::routes::api_routes::create_api_router;
use axum::{Router, routing::get};
use dotenvy::dotenv;
use routes::web_routes::create_web_router;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 1. Cargar .env
    dotenv().ok();

    // 2. Configurar la conexión a la base de datos
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // 3. Ejecutar migraciones
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run migrations");

    // 4. Configurar el estado compartido
    let app_state = state::AppState { db_pool };

    // 5. Configurar las rutas
    let app = Router::new()
        .route("/", get(web_handlers::index_handler))
        .merge(create_web_router())
        .merge(create_api_router())
        .with_state(app_state);

    // 6. Iniciar el servidor
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Servidor escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
