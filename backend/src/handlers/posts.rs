use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, QueryOrder, ActiveModelTrait};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{AppState, models::post};

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub summary: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub summary: Option<String>,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<post::Model> for PostResponse {
    fn from(model: post::Model) -> Self {
        PostResponse {
            id: model.id,
            title: model.title,
            slug: model.slug,
            content: model.content,
            summary: model.summary,
            status: model.status,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

pub async fn list_posts(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<PostResponse>> {
    let posts = post::Entity::find()
        .order_by_desc(post::Column::CreatedAt)
        .all(&state.db)
        .await
        .unwrap_or_default();

    Json(posts.into_iter().map(|p| p.into()).collect())
}

pub async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<PostResponse>, StatusCode> {
    let post = post::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(post.into()))
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreatePostRequest>,
) -> Result<Json<PostResponse>, StatusCode> {
    let new_post = post::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        title: sea_orm::ActiveValue::Set(req.title),
        slug: sea_orm::ActiveValue::Set(req.slug),
        content: sea_orm::ActiveValue::Set(req.content),
        summary: sea_orm::ActiveValue::Set(req.summary),
        status: sea_orm::ActiveValue::Set(req.status.unwrap_or_else(|| "draft".to_string())),
        created_at: sea_orm::ActiveValue::NotSet,
        updated_at: sea_orm::ActiveValue::NotSet,
    };

    use sea_orm::ActiveModelTrait;
    let post = new_post
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(post.into()))
}

pub async fn update_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<PostResponse>, StatusCode> {
    let mut post: post::ActiveModel = post::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into();

    if let Some(title) = req.title {
        post.title = sea_orm::ActiveValue::Set(title);
    }
    if let Some(slug) = req.slug {
        post.slug = sea_orm::ActiveValue::Set(slug);
    }
    if let Some(content) = req.content {
        post.content = sea_orm::ActiveValue::Set(content);
    }
    if let Some(summary) = req.summary {
        post.summary = sea_orm::ActiveValue::Set(Some(summary));
    }
    if let Some(status) = req.status {
        post.status = sea_orm::ActiveValue::Set(status);
    }

    let post = post
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(post.into()))
}

pub async fn delete_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<(), StatusCode> {
    let post: post::ActiveModel = post::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into();

    post.delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
