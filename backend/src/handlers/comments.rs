use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, QueryOrder, ActiveModelTrait, QueryFilter, ColumnTrait, TryIntoModel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{AppState, models::comment};

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: i32,
    pub post_id: i32,
    pub author_id: i32,
    pub author_name: String,
    pub content: String,
    pub created_at: Option<String>,
}

pub async fn list_comments(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
) -> Json<Vec<CommentResponse>> {
    let comments = comment::Entity::find()
        .filter(comment::Column::PostId.eq(post_id))
        .order_by_asc(comment::Column::CreatedAt)
        .all(&state.db)
        .await
        .unwrap_or_default();

    // TODO: 关联查询用户名
    Json(comments.into_iter().map(|c| CommentResponse {
        id: c.id,
        post_id: c.post_id,
        author_id: c.author_id,
        author_name: format!("User{}", c.author_id),
        content: c.content,
        created_at: c.created_at,
    }).collect())
}

pub async fn create_comment(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
    Path(post_id): Path<i32>,
    Json(req): Json<CreateCommentRequest>,
) -> Result<Json<CommentResponse>, StatusCode> {
    let new_comment = comment::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        post_id: sea_orm::ActiveValue::Set(post_id),
        author_id: sea_orm::ActiveValue::Set(user_id),
        content: sea_orm::ActiveValue::Set(req.content),
        created_at: sea_orm::ActiveValue::NotSet,
    };

    let comment = new_comment
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CommentResponse {
        id: comment.id,
        post_id: comment.post_id,
        author_id: comment.author_id,
        author_name: format!("User{}", comment.author_id),
        content: comment.content,
        created_at: comment.created_at,
    }))
}

pub async fn delete_comment(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
    Path(id): Path<i32>,
) -> Result<(), StatusCode> {
    let comment: comment::ActiveModel = comment::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into();

    // 只有评论作者可以删除自己的评论
    let comment_model = comment.clone().try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if comment_model.author_id != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    comment
        .delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
