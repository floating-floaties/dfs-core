#![forbid(unsafe_code)]

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use actix::{Actor, Addr};
use futures::future::{ok, err, Ready};
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, FromRequest};
use actix_web::{dev::Service as _};
use futures_util::future::FutureExt;
use actix_cors::Cors;
use actix_web::dev::Payload;
use actix_web::Error as ActixWebError;
use actix_web::error::{ErrorNotFound};
use actix_web::middleware::Logger as AuditLogger;
use actix_web_actors::ws;
use clap::builder::Str;
use env_logger::{Builder};
use oauth2::http::HeaderValue;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use reqwest::{Error, Response};
use reqwest::header::{HeaderName};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::Val;
use uuid::Uuid;
// use dfs_ml::bert::prelude::*;

#[macro_use]
mod config;
mod core;
mod chat_app;
// mod ml;
mod token;
mod openai;

use crate::{
    core::spec,
    config::*,
};
use crate::chat_app::{server, session};

const TRACE_ID: &str = "x-trace-id";
const SPAN_ID: &str = "x-span-id";

macro_rules! trace_id_from_req {
    ($req: expr) => {{
        let tid = $req
            .headers()
            .get(TRACE_ID)
            .cloned()
            .map(|value| {
                match value.to_str() {
                    Ok(v) => v.to_string(),
                    Err(_) => "<Failed :: No Context>".to_string(),
                }
            })
            .unwrap_or_else(|| {
                let uuid = Uuid::new_v4();
                uuid.to_string()
            });
        let sid = $req
            .headers()
            .get(SPAN_ID)
            .cloned()
            .map(|value| {
                match value.to_str() {
                    Ok(v) => v.to_string(),
                    Err(_) => "<Failed :: No Context>".to_string(),
                }
            })
            .unwrap_or_else(|| {
                let uuid = Uuid::new_v4();
                uuid.to_string()
            });

        (tid, sid)
    }};
}

/// Entry point for our websocket route
// async fn chat_route(
//     req: HttpRequest,
//     stream: web::Payload,
//     srv: web::Data<Addr<server::ChatServer>>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     ws::start(
//         session::WsChatSession {
//             id: 0,
//             hb: Instant::now(),
//             room: "main".to_owned(),
//             name: None,
//             addr: srv.get_ref().clone(),
//         },
//         &req,
//         stream,
//     )
// }

/// Displays state
async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}

// #[post("/qa")]
// async fn test_qa(req_body: String) -> web::Json<Value> {
//     let req = serde_json::from_str::<Vec<QaInput>>(req_body.as_str());
//
//     match req {
//         Ok(req) => {
//             let answers = dfs_ml::bert::ul::qa(&req);
//             web::Json(serde_json::json! {{
//                 "message": "Evaluated QAs".to_string(),
//                 "result": Some(answers),
//                 "error": false,
//             }})
//         }
//         Err(error) => {
//             let message = format!("Failed to parse request: {:?}", error.to_string());
//             web::Json(serde_json::json! {{
//                 "message": message,
//                 "result": null,
//                 "error": true,
//             }})
//         }
//     }
// }
#[derive(Serialize, Deserialize, Debug)]
struct ChatbotRequest {
    hist: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ChatbotResponse {
    response: Option<openai::isla::ChatbotResponse>,
    error: bool,
    message: String,
}


#[post("/isla-response")]
async fn chatbot(req_body: String) -> web::Json<ChatbotResponse> {
    let req = serde_json::from_str::<ChatbotRequest>(req_body.as_str());

    if req.is_err() {
        return web::Json(ChatbotResponse {
            error: true,
            response: None,
            message: format!("Failed to parse incoming request: {req:?}")
        })
    }

    if let Some(Some(config)) = global!() {
        // checked if is error
        let req = req.unwrap();
        let res = openai::isla::get_response(
            &config,
            req.hist
        ).await;

        match res {
            Ok(res) => {
                web::Json(ChatbotResponse {
                    error: false,
                    response: Some(res),
                    message: "".into()
                })
            },
            Err(err) => {
                web::Json(ChatbotResponse {
                    error: true,
                    response: None,
                    message: format!("Failed to get response from bot: {err:?}")
                })
            }
        }
    } else {
        web::Json(ChatbotResponse {
            error: true,
            response: None,
            message: "Failed to get essential settings".into()
        })
    }

}

#[post("/condition")]
async fn test_condition(req_body: String) -> web::Json<spec::web::ConditionResponse> {
    // if let Some(_global) = global!() {
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
    // } else {
    //     web::Json(spec::web::ConditionResponse {
    //         message: "Failed to lock global resource".into(),
    //         result: None,
    //         error: true,
    //     })
    // }
}

#[get("/")]
async fn home() -> impl Responder {
    let msg = if let Some(Some(g)) = global!() {
        g.config.message
    } else {
        "How do you do?".to_string()
    };
    HttpResponse::Ok().body(msg)
}

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok()
        .body(
            std::fs::read_to_string("build-date.txt")
                .unwrap_or_else(|_| "unknown".into())
        )
}

#[post("/version")]
async fn version_post() -> impl Responder {
    let build_date = std::fs::read_to_string("build-date.txt")
        .unwrap_or_else(|_| "unknown".into());

    let build_date = serde_json::json!({
        "version": build_date,
    });
    let build_date = serde_json::to_string(&build_date);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(build_date.unwrap_or_default())
}

#[derive(Debug, serde::Deserialize)]
struct Thing;

impl FromRequest for Thing {
    type Error = ActixWebError;
    type Future = Ready<Result<Thing, ActixWebError>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if is_authorized(req) {
            ok(Thing {})
        } else {
            err(ErrorNotFound("Not Found"))
        }
    }
}

fn is_authorized(req: &HttpRequest) -> bool {
    let headers = req.headers();
    if let Some(secret) = headers.get("X-Hub-Signature-256") {
        let github_event = headers.get("X-GitHub-Event");
        let github_delivery = headers.get("X-GitHub-Event");
        let user_agent = headers.get("User-Agent");
        let (trace_id, span_id) = trace_id_from_req!(req);

        let headers_present = {
            let opts = vec![
                &github_event,
                &github_delivery,
                &user_agent,
            ];
            let count = opts.len();
            let valid_count = opts
                .iter()
                .filter(|x| x.is_some())
                .count();

            count == valid_count && valid_count > 0 && count > 0
        };

        let valid = {
            let matched = match secret.to_str() {
                Ok(secret) => {
                    if let Some(Some(g)) = global!() {
                        log::info!("Comparing secrets; <? trace_id={trace_id:?} span_id={span_id:?} ?>");
                        g.config.cmp_webhook_secret(secret)
                    } else {
                        log::error!("Failed to get global config for reload; <? trace_id={trace_id:?} span_id={span_id:?} ?>");
                        false
                    }
                }
                Err(_) => false,
            };

            let valid_user_agent = match user_agent {
                None => false,
                Some(user_agent) => {
                    user_agent.to_str().unwrap_or("invalid")
                        .starts_with("GitHub-Hookshot/")
                }
            };

            matched && headers_present && valid_user_agent
        };

        if valid {
            log::warn!("Reload Attempt: req={req:?}; <? trace_id={trace_id:?} span_id={span_id:?} ?>");

            std::process::Command::new("sh start.sh")
                .spawn()
                .expect("sh command failed to start");
        }
    }

    false
}

/// extract `Thing` from request
async fn index(supplied_thing: Result<Thing, ActixWebError>) -> String {
    match supplied_thing {
        Ok(thing) => format!("Got thing: {:?}", thing),
        Err(e) => format!("Unknown error: {}", e)
    }
}

// #[post("/token")]
// async fn token(
//     body,
// ) -> impl Responder {
//     HttpResponse::Ok().body(
//         std::fs::read_to_string("build-date.txt")
//         .unwrap_or("unknown".to_string())
//     )
// }

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct LogDetails {
    time_format: String,
    time: String,
    timestamp: i64,
    local_time: String,
    level: log::Level,
    message: String,
    target: String,
    is_audit: bool,
    module: Option<String>,
    lineno: Option<u32>,
    file: Option<String>,
    app_name: String,
    trace_id: Option<String>,
    span_id: Option<String>,
    process_id: String,
    thread_id: String,
    thread_name: String,
    stdout: Option<String>,
    human_readable: Option<String>,
    logger_format: String,
}

impl LogDetails {
    fn process_information() -> (String, String, String) {
        let process_id = std::process::id().to_string();
        let current_thread = std::thread::current();
        let thread_name = current_thread
            .name()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "no-thread-name".to_string());
        let thread_id = format!("{:?}", current_thread.id());

        (process_id, thread_id, thread_name)
    }
    fn get_mapping(&self) -> Vec<(&'static str, String)> {
        vec![
            ("%(lineno)", self.lineno.map(|v| v.to_string())
                .unwrap_or_else(|| "<No Lineno>".to_string())),
            ("%(trace_id)", self.trace_id.clone().unwrap_or_else(|| "<No Context>".to_string())),
            ("%(span_id)", self.span_id.clone().unwrap_or_else(|| "<No Context>".to_string())),
            ("%(utctime)", self.time.clone()),
            ("%(localtime)", self.local_time.clone()),
            ("%(level)", self.level.clone().to_string()),
            ("%(app_name)", self.app_name.clone()),
            ("%(message)", self.message.clone()),
            ("%(target)", self.target.clone()),
            ("%(is_audit)", self.is_audit.clone().to_string()),
            ("%(module)", self.module.clone().unwrap_or_else(|| "<No Module>".to_string())),
            ("%(file)", self.file.clone().unwrap_or_else(|| "<No File>".to_string())),
            ("%(pid)", self.process_id.clone()),
            ("%(thread_name)", self.thread_name.clone()),
            ("%(thread_id)", self.thread_id.clone()),
        ]
    }

    fn set_log_ids(&mut self) -> Self {
        if self.is_audit {
            if let Ok(value) = serde_json::from_str::<Value>(&self.message) {
                if let Some(obj) = value.as_object() {
                    if let Some(id) = obj.get("spanId") {
                        if let Some(id) = id.as_str() {
                            self.span_id = Some(id.to_string());
                        }
                    }

                    if let Some(id) = obj.get("traceId") {
                        if let Some(id) = id.as_str() {
                            self.trace_id = Some(id.to_string());
                        }
                    }
                }
            }
        }

        self.clone()
    }

    fn set_human_readable(&mut self) -> Self {
        let mut result = self.logger_format.clone();
        self
            .get_mapping()
            .iter()
            .for_each(|(k, v)| {
                result = result.replace(*k, v);
            });

        self.human_readable = Some(result);
        self.clone()
    }

    fn set_stdout(&mut self) -> Self {
        use colored::Colorize;

        let mut result = self.logger_format.clone();
        self
            .get_mapping()
            .iter()
            .for_each(|(k, v)| {
                let key = *k;
                if key == "%(level)" {
                    let v = match self.level {
                        log::Level::Error => { v.red().bold().to_string() }
                        log::Level::Warn => { v.yellow().bold().to_string() }
                        log::Level::Info => { v.green().bold().to_string() }
                        log::Level::Debug => { v.blue().bold().to_string() }
                        log::Level::Trace => { v.yellow().italic().to_string() }
                    };

                    result = result.replace(*k, &v);
                } else if key == "%(message)" {
                    let v = v.bright_blue().to_string();
                    result = result.replace(*k, &v);
                } else {
                    result = result.replace(*k, v);
                }
            });

        self.stdout = Some(result);
        self.clone()
    }
}

async fn dump_log_messages(config: Global, to_dump: Vec<(i64, String)>, is_timeout: bool) -> bool {
    if to_dump.is_empty() {
        return false;
    }

    // just in case nothing ever gets cleared
    if to_dump.len() > 1000 {
        return true;
    }

    if is_timeout || to_dump.len() >= 100 {
        if let Some(global_config) = Some(config.clone()) {
            let env = global_config.env;
            let app_name = env.config_details.app_name;
            let email = env.config_details.email;
            let client = reqwest::Client::new();
            let timestamp_events = to_dump
                .iter()
                .map(|(t, _v)| t.to_string())
                .collect::<Vec<String>>();
            let log_events = to_dump
                .iter()
                .map(|(_t, v)| v.to_string())
                .collect::<Vec<String>>();
            let payload = serde_json::json! {{
                "action": "PutLog",
                "app": app_name,
                "email": email,
                "timestamp_events": timestamp_events,
                "log_events": log_events,
            }};

            let response = client
                .post(env.config_details.url)
                .header("x-api-key", env.config_details.api_key)
                .json(&payload)
                .send()
                .await;

            let _ = match response {
                Ok(res) => Some(res),
                Err(response_error) => {
                    println!("Failed to get response from config server: {response_error:?}");
                    None
                }
            };
        }

        true
    } else {
        false
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Global::new().await;
    config.update_mutex(true).await;

    let (tx, rx) = std::sync::mpsc::channel::<LogDetails>();
    let tx_mutex = Mutex::new(tx.clone());
    let tx_fallback_config = config.clone();

    let _log_manager = tokio::spawn(async move {
        let tx_config_clone = tx_fallback_config.clone();
        let mut dumps = Vec::<(i64, String)>::new();

        loop {
            let mut recv_timeout = false;
            let one_min = std::time::Duration::from_secs(60);
            let cnf = global!()
                .unwrap_or_else(|| Some(tx_config_clone.clone()))
                .unwrap_or_else(|| tx_config_clone.clone());

            match rx.recv_timeout(one_min) {
                Ok(mut message) => {
                    let message = message
                        .set_log_ids()
                        .set_human_readable()
                        .set_stdout();

                    let r_message = std::rc::Rc::new(&message);
                    let hr = &r_message.human_readable;
                    let terminal = &r_message.stdout;

                    if let Some(terminal) = terminal {
                        println!("{}", terminal);
                    } else if let Some(line) = hr {
                        println!("{}", line);
                    } else {
                        println!("{:?}", message);
                    }

                    if cnf.env.save_logs {
                        if let Ok(dump) = serde_json::to_string(&message) {
                            dumps.push((message.timestamp, dump));
                        }
                    }
                }
                Err(_) => {
                    recv_timeout = true;
                }
            }

            if cnf.env.save_logs {
                let dumps_clone = dumps.clone();
                let result = dump_log_messages(
                    cnf.clone(),
                    dumps_clone,
                    recv_timeout,
                ).await;

                if result {
                    dumps.clear();
                }
            }
        }
    });
    let config_copy = config.clone();
    let mut builder = Builder::from_default_env();
    builder
        .format(move |_buf, record| {
            let config_clone = global!()
                .unwrap_or_else(|| Some(config_copy.clone()))
                .unwrap_or_else(|| config_copy.clone());

            let configuration = config_clone.config;
            let env = config_clone.env;

            let target = record.metadata().target();
            let args = record.args();
            let module_path = record.module_path().map(|v| v.to_string());
            let file = record.file().map(|v| v.to_string());
            let line = record.line();
            let level = record.level();
            let log_message = args.to_string();
            let is_audit = target == "audit";
            let local_time = chrono::offset::Local::now();
            let utc_time = chrono::offset::Utc::now();

            let (pid, tid, thread_name) = LogDetails::process_information();
            let message = LogDetails {
                time_format: configuration.time_format.to_string(),
                time: utc_time.format(&configuration.time_format).to_string(),
                local_time: local_time.format(&configuration.time_format).to_string(),
                timestamp: utc_time.timestamp(),
                target: target.to_string(),
                app_name: env.config_details.app_name,
                message: log_message,
                module: module_path,
                lineno: line,
                trace_id: None,
                span_id: None,
                process_id: pid,
                thread_id: tid,
                logger_format: configuration.service_logger_format,
                human_readable: None,
                stdout: None,
                thread_name,
                level,
                file,
                is_audit,
            };

            match tx_mutex.lock() {
                Ok(sender) => {
                    if let Err(error) = sender.send(message) {
                        eprintln!("MPSCERROR: Failed to send message: {error:?}");
                    };
                }
                Err(lock_error) => {
                    eprintln!("MPSCERROR: Failed getting lock for sender: lock_error={lock_error:?}");
                }
            };

            Ok(())
        })
        .filter(None, log::LevelFilter::Info)
        .init();


    log::info!("Stating application: {:?}", config.env.host_port());

    let app_state = Arc::new(AtomicUsize::new(0));
    let server = server::ChatServer::new(app_state.clone()).start();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(move || {
        let conf = global!().unwrap().unwrap();
        let environment = conf.env.clone();
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
            .wrap_fn(|req, srv| {
                let req_head = req.headers().clone();
                srv.call(req).map(move |res| {
                    if let Ok(mut response) = res {
                        let headers = response.headers_mut();
                        for static_value in [TRACE_ID, SPAN_ID] {
                            let id = match req_head.get(static_value) {
                                None => {
                                    let uuid = Uuid::new_v4();
                                    let uuid_string = uuid.to_string();
                                    let uuid_bytes = uuid_string.as_bytes();
                                    HeaderValue::from_bytes(uuid_bytes)
                                }
                                Some(existing_id) => {
                                    Ok(existing_id.clone())
                                }
                            };

                            if let Ok(id) = id {
                                // println!("{:?}: {:?}", static_value, id);
                                let key = HeaderName::from_static(static_value);
                                headers.insert(key, id);
                            }
                        }

                        return Ok(response);
                    }

                    res
                })
            })

            .wrap(AuditLogger::new(&conf.config.audit_logger_format).log_target("audit"))
            .wrap(cors)
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .service(home)
            .route("/count", web::get().to(get_count))
            // .route("/ws", web::get().to(chat_route))
            .route("/update", web::post().to(index))
            // .route("/qa", web::post().to(test_qa))
            .service(chatbot)
            .service(test_condition)
            .service(version)
            .service(version_post)
            .service(token::token)
    })
        // .bind(config.env.host_port())?
        .bind_openssl("0.0.0.0:443", builder)?
        .workers(5)
        .run()
        .await
}
