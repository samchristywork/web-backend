use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[get("/api/tags")]
async fn tags() -> impl Responder {
    let contents = fs::read_to_string("content/tags").expect("Something went wrong while reading the file");
    HttpResponse::Ok().json(Message {
        message: contents
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address = "127.0.0.1:8080";

    println!("Starting server at {}", bind_address);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(tags)
    })
    .bind(bind_address)?
    .run()
    .await
}
