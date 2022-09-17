#![forbid(unsafe_code)]

use actix_cors::Cors;
use actix_web::{
    get, http, post, web, App, HttpResponse, HttpServer, Responder
};

// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod core;
use crate::core::spec;

// use crate::core::spec::Spec;


#[post("/condition")]
async fn test_condition(req_body: String) -> web::Json<spec::web::ConditionResponse> {
    println!("{}", req_body.as_str());
    let req = serde_json::from_str::<spec::web::ConditionRequest>(req_body.as_str());

    match req {
        Ok(req) => {
            let evaluated = req.spec.format_eval_for_response(req.condition);
            match evaluated {
                Ok(value) => web::Json(spec::web::ConditionResponse {
                    message: "Evaluated expression".into(),
                    result: Some(value),
                    error: false,
                }),
                Err(message) => web::Json(spec::web::ConditionResponse {
                    message,
                    result: None,
                    error: true,
                }),
            }
        }
        Err(error) => {
            let message = format!("Failed to parse json: {:?}", error.to_string());
            web::Json(spec::web::ConditionResponse {
                message,
                result: None,
                error: true,
            })
        }
    }
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("How do you do?")
}

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok()

        .insert_header(("Access-Control-Allow-Origin", "*"))
    .body(
        std::fs::read_to_string("build-date.txt")
        .unwrap_or_else(|_| "unknown".into())
    )
}

// #[post("/token")]
// async fn token(
//     body
// ) -> impl Responder {
//     HttpResponse::Ok().body(
//         std::fs::read_to_string("build-date.txt")
//         .unwrap_or("unknown".to_string())
//     )
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host: String = match std::env::var("HOST") {
        Ok(v) => v,
        _ => "0.0.0.0".into(),
    };
    let port: u16 = match std::env::var("PORT") {
        Ok(v) => v.parse::<u16>().expect("Invalid PORT number was provided"),
        _ => 80,
    };

    let domain: String = match std::env::var("DOMAIN") {
        Ok(v) => v,
        _ => "floatingfloaties.com".into(),
    };

    let urls = [
        "http://localhost:8080".into(),
        format!("http://localhost:{port}"),
        format!("http://{host}:8080"),
        format!("http://{host}:{port}"),
        format!("https://{domain}"),
        format!("https://dev.{domain}"),
        format!("https://qa.{domain}"),
        format!("https://release.{domain}"),
    ];

    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();
    // let config = aws_config::load_from_env().await;

    HttpServer::new(|| {
        let env: String = match std::env::var("ENV") {
            Ok(v) => v,
            _ => "development".to_owned(),
        };

        let cors = if env.to_ascii_lowercase() == "development" {
            Cors::permissive()
                .allowed_origin("http://0.0.0.0:3000")
                .allowed_origin("http://0.0.0.0:8080")
                .allowed_origin("http://localhost:3000")
                .allowed_origin("http://localhost:8080")
                .allowed_origin("http://127.0.0.1:3000")
                .allowed_origin("http://127.0.0.1:8080")
        } else {
            Cors::default()
                .allowed_origin("https://floatingfloaties.com")
                .allowed_origin("https://dev.floatingfloaties.com")
                .allowed_origin("https://qa.floatingfloaties.com")
                .allowed_origin("https://release.floatingfloaties.com")
        };

        let cors = cors
            .send_wildcard()
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
            .service(version)
            // .service(token)
    })
    // .bind_openssl(format!("{}:{}", host, port), builder)?
    .bind((host, port))?
    .workers(2)
    .run()
    .await
}
