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
    let mut contents = String::new();
    for entry in fs::read_dir("web-content/posts/published")
        .expect("Something went wrong while reading the directory")
    {
        let entry = entry.expect("Something went wrong while reading the entry");
        let path = entry.path();
        let file = fs::read_to_string(path).expect("Something went wrong while reading the file");
        let lines: Vec<&str> = file.split('\n').collect();

        contents += lines[2]
            .split(':')
            .last()
            .expect("String formatted incorrectly")
            .trim()
            .replace(", ", "\n")
            .as_str();
        contents += "\n";
    }
    HttpResponse::Ok().json(Message { message: contents })
}

#[get("/api/posts/{id}")]
async fn posts(id: web::Path<String>) -> impl Responder {
    let whitelist = "abcdefghijklmnopqrstuvwxyz0123456789-";

    let mut sanitized = id;
    sanitized.retain(|c| whitelist.contains(c));
    let contents = fs::read_to_string(format!("web-content/posts/published/{sanitized}.html"))
        .expect("Something went wrong while reading the file");
    HttpResponse::Ok().json(Message { message: contents })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address = "127.0.0.1:8080";

    println!("Starting server at {bind_address}");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new().wrap(cors).service(tags).service(posts)
    })
    .bind(bind_address)?
    .run()
    .await
}
