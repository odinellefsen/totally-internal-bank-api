mod routes;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Process is running");
    HttpServer::new(|| App::new().configure(routes::config))
        // Single-threaded: exactly 1 worker (OS) thread
        .workers(1)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
