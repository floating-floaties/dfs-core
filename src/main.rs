#![forbid(unsafe_code)]
use std::sync::Arc;

use actix_web::{
    get, http, post, web, App,
    HttpResponse, HttpServer, Responder,
};
use actix_cors::Cors;


#[macro_use]
mod config;
mod core;

use crate::{
    core::spec,
    config::*
};

#[post("/condition")]
async fn test_condition(req_body: String) -> web::Json<spec::web::ConditionResponse> {
    if let Some(global) = global! () {
        println!("global={global:?}");
        println!("req={}", req_body.as_str());
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
    } else {
        web::Json(spec::web::ConditionResponse {
            message: "Failed to lock global resource".into(),
            result: None,
            error: true,
        })
    }
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("How do you do?")
}

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok()
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
    let env = Environment::from_env();

    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();
    // let config = aws_config::load_from_env().await;

    HttpServer::new(move || {
        let environment = Environment::from_env();

        let cors = if environment.is_dev() {
            let protocols = ["https", "http", "ws", "wss"];
            let domains = ["0.0.0.0", "localhost", "127.0.0.1"];
            let ports = ["3000", "8080", "8000", "80", "443"];
            let mut cors = Cors::permissive();

            for protocol in protocols {
                for domain in domains {
                    for port in ports {
                        let origin = format!("{protocol}://{domain}:{port}");
                        cors = cors.allowed_origin(origin.as_str());

                    }
                }
            }

            cors
        } else {
            Cors::default()
                .allowed_origin("https://floatingfloaties.com")
                .allowed_origin("https://dev.floatingfloaties.com")
                .allowed_origin("https://qa.floatingfloaties.com")
                .allowed_origin("https://release.floatingfloaties.com")
        };

        let cors = cors
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
        .bind(env.host_port())?
        .workers(2)
        .run()
        .await
}
