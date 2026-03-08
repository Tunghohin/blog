use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

const SECRET_KEY: &str = "your-secret-key-change-in-production";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // user id
    pub username: String,
    pub role: String,
    pub exp: usize,
}

pub async fn auth_middleware(
    _state: State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(header) => header.strip_prefix("Bearer ").unwrap_or(header),
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let config = Validation::default();
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY.as_bytes()),
        &config,
    ) {
        Ok(token_data) => {
            let mut req = req;
            req.extensions_mut().insert(token_data.claims.sub);
            req.extensions_mut().insert(token_data.claims.username);
            req.extensions_mut().insert(token_data.claims.role);
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub fn create_token(user_id: i32, username: String, role: String) -> Result<String, jsonwebtoken::errors::Error> {
    use chrono::{Utc, Duration};

    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        username,
        role,
        exp: expiration,
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    )
}
