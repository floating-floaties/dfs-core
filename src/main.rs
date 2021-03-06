use std::path::PathBuf;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{
    get, http, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

mod core;
use crate::core::spec;
// use crate::core::spec::Spec;

async fn static_route(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    return Ok(NamedFile::open(path)?);
}

#[post("/condition")]
async fn test_condition(req_body: String) -> web::Json<spec::web::ConditionResponse> {
    println!("{}", req_body.as_str());
    let req = serde_json::from_str::<spec::web::ConditionRequest>(req_body.as_str());
    
    match req {
        Ok(req) => {
            let evalutated = req.spec.format_eval_for_response(req.condition);
            match evalutated {
                Ok(value) => {
                    web::Json(spec::web::ConditionResponse {
                        message: "Evaluated expression".to_string(),
                        result: Some(value),
                        error: false,
                    })
                },
                Err(message) => {
                    web::Json(spec::web::ConditionResponse {
                        message,
                        result: None,
                        error: true,
                    })
                }
            }
        },
        Err(error) => {
            let err_msg = format!("Failed to parse json: {:?}", error.to_string());
            web::Json(spec::web::ConditionResponse {
                message: err_msg,
                result: None,
                error: true,
            })
        },
    }
}


#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("How do you do?")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host: String = match std::env::var("HOST") {
        Ok(v) => v,
        _ => "0.0.0.0".to_string(),
    };
    let port: u16 = match std::env::var("PORT") {
        Ok(v) => v.parse::<u16>().unwrap(),
        _ => 8080,
    };

    HttpServer::new(|| {
        let env: String = match std::env::var("ENV") {
            Ok(v) => v,
            _ => "production".to_owned(),
        };

        let mut cors = if env == "development" {
            Cors::permissive().allowed_origin("http://localhost:19006")
        } else {
            Cors::default().allowed_origin("https://floaties.dudi.win")
        };

        cors = cors
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(http::header::ACCEPT)
            .allowed_header(http::header::AUTHORIZATION)
            .allowed_header(http::header::CONTENT_LENGTH)
            .allowed_header(http::header::HOST)
            .allowed_header(http::header::CONTENT_TYPE)
            .allowed_header(http::header::USER_AGENT)
            .allowed_header(http::header::ACCEPT_ENCODING)
            .allowed_header(http::header::CONNECTION)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(home)
            .service(test_condition)
            .route("/{filename:.*}", web::get().to(static_route))
    })
    .bind((host, port))?
    .workers(2)
    .run()
    .await
}
