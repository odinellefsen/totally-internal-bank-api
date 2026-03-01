mod routes;

use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let db_pool = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .configure(routes::config)
    })
    // Single-threaded: exactly 1 worker (OS) thread (Node.js style)
    .workers(1)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
