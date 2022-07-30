use actix_web::{App, HttpServer, web};
use rnd_is::api::{number, numbers, home, uuid, serve_openapi_spec, ping};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::head().to(ping))
            .route("/", web::get().to(home))
            .route("/openapi", web::get().to(serve_openapi_spec))
            .route("/number", web::get().to(number))
            .route("/numbers", web::get().to(numbers))
            .route("/uuid", web::get().to(uuid))
    })
        .bind(("0.0.0.0", resolve_port()))?
        .run()
        .await
}

fn resolve_port() -> u16 {
    match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => panic!("Environment variable PORT not set"),
    }
}
