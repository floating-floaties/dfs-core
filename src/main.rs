use std::path::PathBuf;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{http::header::ContentType};

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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct ConditionResponse {
    message: String,
    error: bool,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!")
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
async fn test_condition(req_body: String) -> web::Json<ConditionResponse> {
    let mut spec = Spec::default();
    let req = serde_json::from_str::<ConditionRequest>(req_body.as_str());
    println!("Req body: {}", req_body);
    println!("Req parsed: {:?}", req);
    if req.is_err() {

        // return HttpResponse::InternalServerError().body(serde_json::json!({
        //     "message": "Failed to parse json",
        //     "error": true,
        // }));
        let err_msg = format!("Failed to parse json: {:?}", req);
        // return HttpResponse::InternalServerError().body(err_msg);

        // return HttpResponse::Ok()
        // // .content_type(ContentType::)
        // .insert_header(("Content-Type", "application/json"))
        // .json(serde_json::json!({
        //     "result": err_msg,
        //     "error": true,
        // }))

        return web::Json(ConditionResponse {
            message: err_msg,
            error: true,
        });
    }

    let req = req.unwrap();
    // log::info!("Request: {:?}", req);
    println!("Request Spec context: {:?}", req.spec.context);
    println!("Request Spec system: {:?}", req.spec.system);
    println!("Codition: {:?}", req.condition);

    spec.context = req.spec.context;
    spec.system = req.spec.system;

    let result = spec.eval(req.condition);
    let res = format!("Result: {:?}", result);

    // HttpResponse::Ok()
    // .insert_header(("Content-Type", "application/json"))
    // .json(serde_json::json!({
    //     "result": res,
    //     "error": false,
    // }))

    return web::Json(ConditionResponse {
        message: res,
        error: true,
    });
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
        // let cors = Cors::permissive()
        //     .allowed_origin("https://floaties.dudi.win")
        //     .allowed_methods(vec!["GET", "POST"])
        //     .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        //     .allowed_header(header::CONTENT_TYPE)
        //     .allowed_header(header::ACCESS_CONTROL_ALLOW_ORIGIN)
        //     .max_age(3600);
        
        App::new()
            // .wrap(cors)
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