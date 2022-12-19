use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use serde_json::json;

#[derive(Eq, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default, Hash)]
pub struct GlobalCognitoConfig {
    pub id: String,
    pub secret: String,
    pub auth_url: String,
    pub token_url: String,
}

#[derive(PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct IslaSettings {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32
}

#[derive(PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct DustinDiazIoConfig {
    #[serde(alias="apiUrl")]
    pub api_url: HashMap<String, String>,
}


#[derive(PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GlobalConfig {
    version: String,
    pub(crate) audit_logger_format: String,
    pub(crate) service_logger_format: String,
    pub(crate) time_format: String,
    pub(crate) motd: String,

    #[serde(alias="dustindiaz.io")]
    pub(crate) dustindiaz_io: DustinDiazIoConfig,
    reload_events: Vec<String>,
    github_secret: String,
    concord_secret: String,
    error: bool,
    pub(crate) cognito: GlobalCognitoConfig,
    pub(crate) openai_secret: String,
    pub(crate) isla_settings: IslaSettings,
}

impl GlobalConfig {
    async fn new() -> Self {
        let env = Environment::from_env();
        let app_name = env.config_details.app_name;
        let email = env.config_details.email;
        let client = reqwest::Client::new();

        let payload = json!({
            "action": "GetConfig",
            "app": app_name,
            "email": email
        });
        println!("Fetching config: query={:?}", payload);
        let response = client
            .post(env.config_details.url)
            .header("x-api-key", env.config_details.api_key)
            .json(&payload)
            .send()
            .await;

        println!("{response:?}");
        let config = match response {
            Ok(res) => {
                match res.json::<Self>().await {
                    Ok(config) => Some(config),
                    Err(parse_error) => {
                        eprintln!("Failed to parse config: {parse_error:?}");
                        None
                    }
                }
            }
            Err(response_error) => {
                eprintln!("Failed to get response from config server: {response_error:?}");
                None
            }
        };

        if let Some(config) = config {
            println!("Got config from server!");
            return config;
        }

        Self {
            version: "-1".into(),
            audit_logger_format: "".into(),
            service_logger_format: "[%(level)]: %(message)".into(),
            time_format: "%d/%m/%Y %H:%M".into(),
            reload_events: vec![],
            github_secret: "invalid".into(),
            concord_secret: "invalid".into(),
            motd: "[FAIL] How do you do?".into(),
            error: true,
            cognito: Default::default(),
            openai_secret: "<secret>".into(),
            isla_settings: Default::default(),
            dustindiaz_io: Default::default(),
        }
    }

    pub fn cmp_webhook_secret<S: AsRef<str>>(&self, other: S) -> bool {
        let mut map = std::collections::HashSet::with_capacity(5);
        map.insert(self.github_secret.clone());
        map.insert(self.concord_secret.clone());
        map.contains(other.as_ref())
    }

    pub fn is_expected_reload_event<S: AsRef<str>>(&self, ev: S) -> bool {
        self.reload_events.contains(&ev.as_ref().to_string())
    }
}


#[derive(PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Global {
    pub env: Environment,
    pub config: GlobalConfig,
}

impl Global {
    pub(crate) async fn new() -> Self {
        Self {
            config: GlobalConfig::new().await,
            env: Environment::from_env(),
        }
    }

    pub(crate) async fn update_mutex(&self, use_self: bool) -> Option<Self> {
        let g = Arc::clone(&GLOBAL_MUTEX);
        // let handle = tokio::spawn(async {});
        let lock = g.lock();

        match lock {
            Ok(mut obj) => {
                let new_config = if use_self {
                    Some(self.clone())
                } else {
                    Some(Global::new().await)
                };

                *obj = new_config.clone();
                new_config
            },
            Err(err) => {
                eprintln!("ERROR: Failed to acquire global resource lock '{:?}'", err);
                None
            }
        }
    }
}

#[derive(Eq, PartialEq, Clone, serde::Deserialize, serde::Serialize, Hash)]
pub struct ConfigDetails {
    pub url: String,
    pub api_key: String,
    pub app_name: String,
    pub environment: String,
    pub email: String,
}

#[derive(Eq, PartialEq, Clone, serde::Deserialize, serde::Serialize, Hash)]
pub struct Environment {
    pub host: String,
    pub port: u16,
    pub env: String,
    pub config_details: ConfigDetails,
    pub save_logs: bool,
}

fn is_true(var: String) -> bool {
    ["true", "1"].contains(&var.to_lowercase().trim())
}


impl Environment {
    pub(crate) fn from_env() -> Self {
        let host: String = match std::env::var("HOST") {
            Ok(v) => v,
            _ => "0.0.0.0".into(),
        };
        let port: u16 = match std::env::var("PORT") {
            Ok(v) => v.parse::<u16>().expect("Invalid PORT number was provided"),
            _ => 80,
        };

        let env: String = match std::env::var("ENV") {
            Ok(v) => v,
            _ => "dev".to_owned(),
        };

        let config_url: String = std::env::var("CONFIG_CONCORD_URL")
            .expect("Provide Url for config server; export CONFIG_CONCORD_URL");

        let config_api_key: String = std::env::var("CONFIG_CONCORD_API_KEY")
            .expect("Provide Api Key for config server; export CONFIG_CONCORD_API_KEY");

        let config_email: String = std::env::var("CONFIG_CONCORD_EMAIL")
            .expect("Provide Api Key for config server; export CONFIG_CONCORD_EMAIL");

        let mut config_app_name: String = std::env::var("CONFIG_CONCORD_APP_NAME")
            .expect("Provide Api Key for config server; export CONFIG_CONCORD_APP_NAME");

        let save_logs: String = std::env::var("SAVE_LOGS")
            .unwrap_or_else(|_| "false".to_string());

        let save_logs = is_true(save_logs);

        let config_env = env.clone();

        if !config_app_name.ends_with(config_env.as_str()) {
            config_app_name = format!("{config_app_name}-{config_env}");
        }

        let config_details = ConfigDetails {
            url: config_url,
            api_key: config_api_key,
            email: config_email,
            environment: config_env,
            app_name: config_app_name,
        };

        Self {
            host,
            port,
            env,
            config_details,
            save_logs,
        }
    }

    pub(crate) fn host_port(&self) -> (String, u16) {
        (self.host.clone(), self.port)
    }

    pub(crate) fn is_dev(&self) -> bool {
        self.env.to_ascii_lowercase().contains("dev")
    }
}

/// ```rust
/// use std::sync::Arc;
/// use crate::config::*;
///
/// if let Some(Some(g)) = global!() {
///     // use global config
///     let config = g.config;
///     let env = g.env;
/// } else {
///     // failed to get global config
/// }
/// ```
macro_rules! global {
    () => {{
        let g = Arc::clone(&GLOBAL_MUTEX);
        let lock = g.lock();
        match lock {
            Ok(obj) => Some(obj.clone()),
            Err(err) => {
                log::error!("ERROR: Failed to acquire global resource lock '{:?}'", err);
                None
            }
        }
    }};
}

lazy_static! {
    pub static ref GLOBAL_MUTEX: Arc<Mutex<Option<Global>>> = Arc::new(Mutex::new(None));
}
