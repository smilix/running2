use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::post;
use actix_web::web;
use actix_web::web::Json;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::common;
use crate::auth::Claims;
use crate::config::CONFIG;
use crate::errors::ApiError;

#[derive(Deserialize)]
pub struct Login {
    pub user: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SessionResult {
    pub result: String,
    pub session: String,
}

#[post("/auth")]
async fn login(login_data: web::Json<Login>) -> Result<Json<SessionResult>, ApiError> {
    if login_data.user != CONFIG.login {
        info!("Invalid login '{}'.", login_data.user);
        return Err(ApiError::Unauthorized("Invalid credentials".to_string()));
    }

    let success = bcrypt::verify(&login_data.password, &CONFIG.password)
        .map_err(|e| {
            error!("Invalid password hash in config: {}", e);
            ApiError::InternalServerError("Config error".to_string())
        })?;

    if !success {
        info!("Invalid password.");
        return Err(ApiError::Unauthorized("Invalid credentials".to_string()));
    }

    let session_lifetime_sec = &CONFIG.jwt_expiration_in_h * 60 * 60;
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        + session_lifetime_sec;

    let my_claims = Claims {
        sub: login_data.user.clone(),
        exp: ts,
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(&CONFIG.jwt_key.as_bytes()),
    ).unwrap();

    return Ok(Json(SessionResult {
        result: common::SUCCESS_MSG.to_string(),
        session: token,
    }));
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}