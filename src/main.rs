use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod core;
use crate::core::spec::Spec;

const HOST: &str = "0.0.0.0";
const PORT: u16 = 8080;

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
    
    println!("Running on: http://{HOST}:{PORT}");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(greet)
    })
    .bind((HOST, PORT))?
    .workers(4)
    .run()
    .await
}