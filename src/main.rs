use actix_web::{App, HttpServer, web};
use random_api::api::{number};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/number", web::get().to(number))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
