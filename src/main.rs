use std::path::PathBuf;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};

mod core;
use crate::core::spec::Spec;


async fn static_route(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    return Ok(NamedFile::open(path)?);
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    web::Json(req_body)
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {name}!"))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let user_spec = Spec::default();
    println!("{}", user_spec.eval("true && true"));

    let host: String = match std::env::var("HOST") {
        Ok(v) => v,
        _ => "0.0.0.0".to_string(),
    };
    let port: u16 = match std::env::var("PORT") {
        Ok(v) => v.parse::<u16>().unwrap(),
        _ => 8080,
    };
    
    println!("Running on: http://{host}:{port}");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(greet)
            .route("/{filename:.*}", web::get().to(static_route))
    })
    .bind((host, port))?
    .workers(4)
    .run()
    .await
}