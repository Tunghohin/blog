use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, QueryOrder, ActiveModelTrait, QueryFilter, ColumnTrait, TryIntoModel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;

use crate::{AppState, models::{comment, user}};

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CommentResponse {
    pub id: i32,
    pub post_id: i32,
    pub author_id: i32,
    pub author_name: String,
    pub content: String,
    pub parent_id: Option<i32>,
    pub created_at: Option<String>,
    pub replies: Vec<CommentResponse>,
}

pub async fn list_comments(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
) -> Json<Vec<CommentResponse>> {
    // 查询评论并关联查询用户信息
    let comments_with_users = comment::Entity::find()
        .filter(comment::Column::PostId.eq(post_id))
        .order_by_asc(comment::Column::CreatedAt)
        .find_also_related(user::Entity)
        .all(&state.db)
        .await
        .unwrap_or_default();

    // 转换为 CommentResponse
    let all_comments: Vec<CommentResponse> = comments_with_users.into_iter().map(|(c, u)| CommentResponse {
        id: c.id,
        post_id: c.post_id,
        author_id: c.author_id,
        author_name: u.map(|user| user.username).unwrap_or_else(|| format!("User{}", c.author_id)),
        content: c.content,
        parent_id: c.parent_id,
        created_at: c.created_at,
        replies: vec![],
    }).collect();

    // 构建嵌套结构
    Json(build_comment_tree(all_comments))
}

/// 将扁平评论列表转换为树形结构
fn build_comment_tree(comments: Vec<CommentResponse>) -> Vec<CommentResponse> {
    // 按parent_id分组
    let mut by_parent: HashMap<Option<i32>, Vec<CommentResponse>> = HashMap::new();

    for comment in comments {
        by_parent.entry(comment.parent_id).or_default().push(comment);
    }

    // 获取顶级评论（parent_id为None）
    let mut root_comments = by_parent.remove(&None).unwrap_or_default();

    // 递归填充子评论
    for comment in &mut root_comments {
        fill_replies(comment, &by_parent);
    }

    root_comments
}

/// 递归填充评论的回复
fn fill_replies(comment: &mut CommentResponse, by_parent: &HashMap<Option<i32>, Vec<CommentResponse>>) {
    if let Some(mut replies) = by_parent.get(&Some(comment.id)).cloned() {
        for reply in &mut replies {
            fill_replies(reply, by_parent);
        }
        comment.replies = replies;
    }
}

pub async fn create_comment(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
    Extension(username): Extension<String>,
    Path(post_id): Path<i32>,
    Json(req): Json<CreateCommentRequest>,
) -> Result<Json<CommentResponse>, StatusCode> {
    let new_comment = comment::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        post_id: sea_orm::ActiveValue::Set(post_id),
        author_id: sea_orm::ActiveValue::Set(user_id),
        content: sea_orm::ActiveValue::Set(req.content),
        parent_id: sea_orm::ActiveValue::Set(req.parent_id),
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
        author_name: username,
        content: comment.content,
        parent_id: comment.parent_id,
        created_at: comment.created_at,
        replies: vec![],
    }))
}

pub async fn delete_comment(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
    Extension(role): Extension<String>,
    Path(id): Path<i32>,
) -> Result<(), StatusCode> {
    let comment: comment::ActiveModel = comment::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into();

    // 评论作者或管理员可以删除评论
    let comment_model = comment.clone().try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if comment_model.author_id != user_id && role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    comment
        .delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}