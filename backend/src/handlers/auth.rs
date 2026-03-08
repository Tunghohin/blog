use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, QueryFilter, ActiveModelTrait, ColumnTrait};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    AppState,
    middleware::auth::create_token,
    models::user,
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(&req.username))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let valid = bcrypt::verify(&req.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 使用数据库中存储的 role 字段
    let role = user.role.clone();

    let token = create_token(user.id, user.username.clone(), role.clone())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        token,
        username: user.username,
        role: role.to_string(),
    }))
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 新注册用户默认为 normal 角色，admin 只能通过数据库或 init-db 命令创建
    let role = "normal".to_string();

    let user = user::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        username: sea_orm::ActiveValue::Set(req.username.clone()),
        password_hash: sea_orm::ActiveValue::Set(password_hash),
        role: sea_orm::ActiveValue::Set(role.clone()),
        created_at: sea_orm::ActiveValue::NotSet,
    };

    let user = user
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let token = create_token(user.id, user.username.clone(), role.clone())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        token,
        username: user.username,
        role,
    }))
}
