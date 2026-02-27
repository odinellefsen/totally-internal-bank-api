use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bind to localhost:8080
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    // Single-threaded: exactly 1 worker (OS) thread
    .workers(1)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}