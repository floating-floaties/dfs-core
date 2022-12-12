use std::sync::{Arc, Mutex};
use crate::config::GLOBAL_MUTEX;

use actix_web::{web::Form, post, web::Json};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl,
    PkceCodeVerifier,

};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use serde::{Serialize, Deserialize};
use log::{info, error, log};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenBody {
    grant_type: String,
    code: String,
    code_verifier: String,
    redirect_uri: String,
}

#[post("/token")]
pub async fn token(
    body: Form<TokenBody>
) -> Json<oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>> {
    let config = global!().unwrap().unwrap();
    let req = body.into_inner();
    info!("{:?}", req);
    let client = BasicClient::new(
        ClientId::new(config.config.cognito.id),
        Some(ClientSecret::new(config.config.cognito.secret)),
        AuthUrl::new(config.config.cognito.auth_url).expect("failed to build auth url"),
        Some(TokenUrl::new(config.config.cognito.token_url).expect("failed to build token url")),
    ).set_redirect_uri(RedirectUrl::new(req.redirect_uri).expect("failed to build Redirect url"));

    let pkce_verifier = PkceCodeVerifier::new(req.code_verifier);
    let token_result = client.exchange_code(AuthorizationCode::new(req.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await;

    match token_result {
        Err(err) => {
            error!(">>> {:?}", err);
            todo!()
        }
        Ok(val) => {
            info!("Tokens received from OAuth provider! {val:?}");
            Json(val)
        }
    }
}