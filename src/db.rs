use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn init_db_pool(database_url: &str) -> sqlx::Pool<sqlx::Postgres> {
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect(" 🐘 Error al conectar a la base de datos");

    println!("✅ Conexión a la base de datos establecida");

    // 3. Ejecutar migraciones
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("💣 Error al ejecutar las migraciones");

    println!("✅ Migraciones ejecutadas correctamente");

    db_pool
}
