use actix_web::{App, HttpServer, web};
use rnd_is::api::{number, home};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
            .route("/number", web::get().to(number))
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
