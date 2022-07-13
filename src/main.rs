use std::path::PathBuf;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use actix_cors::Cors;
use actix_web::http::header;

mod core;
use crate::core::spec::Spec;


async fn static_route(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    return Ok(NamedFile::open(path)?);
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct ConditionRequest {
    spec: Spec,
    condition: String,
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

#[post("/condition")]
async fn test_condition(req_body: String) -> impl Responder {
    let mut spec = Spec::default();
    let req = serde_json::from_str::<ConditionRequest>(req_body.as_str());

    if req.is_err() {

        // return HttpResponse::InternalServerError().body(serde_json::json!({
        //     "message": "Failed to parse json",
        //     "error": true,
        // }));
        log::error!("Failed to parse json: {:?}", req);
        return HttpResponse::InternalServerError().body("Failed to parse json");
    }

    let req = req.unwrap();
    // log::info!("Request: {:?}", req);
    log::info!("Request Spec context: {:?}", req.spec.context);
    log::info!("Request Spec system: {:?}", req.spec.system);
    log::info!("Codition: {:?}", req.condition);

    spec.context = req.spec.context;
    spec.system = req.spec.system;

    let result = spec.eval(req.condition);
    log::info!("Result: {:?}", result);

    HttpResponse::Ok().body(result.to_string())
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
        let cors = Cors::default()
            .allowed_origin("https://floaties.dudi.win")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .allowed_header(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .max_age(3600);
        
        App::new()
            .wrap(cors)
            .service(hello)
            .service(echo)
            .service(greet)
            .service(test_condition)
            .route("/{filename:.*}", web::get().to(static_route))
    })
    .bind((host, port))?
    .workers(4)
    .run()
    .await
}