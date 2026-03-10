use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

const MAX_FILE_SIZE: usize = 5 * 1024 * 1024; // 5MB
const ALLOWED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "webp"];

#[derive(Debug)]
pub enum UploadError {
    FileTooLarge,
    InvalidFileType,
    WriteError(String),
    NoFile,
}

impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let (status, message): (StatusCode, String) = match self {
            UploadError::FileTooLarge => (StatusCode::PAYLOAD_TOO_LARGE, "文件大小超过限制（最大5MB）".to_string()),
            UploadError::InvalidFileType => (StatusCode::BAD_REQUEST, "不支持的文件类型".to_string()),
            UploadError::WriteError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            UploadError::NoFile => (StatusCode::BAD_REQUEST, "未上传文件".to_string()),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

pub async fn upload_image(mut multipart: Multipart) -> Result<impl IntoResponse, UploadError> {
    // 确保 uploads 目录存在
    let upload_dir = PathBuf::from("uploads");
    if !upload_dir.exists() {
        fs::create_dir_all(&upload_dir)
            .await
            .map_err(|e| UploadError::WriteError(e.to_string()))?;
    }

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| UploadError::WriteError(e.to_string()))?
    {
        let file_name = field
            .file_name()
            .ok_or(UploadError::NoFile)?
            .to_string();

        // 检查文件扩展名
        let extension = file_name
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase();

        if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
            return Err(UploadError::InvalidFileType);
        }

        // 读取文件数据
        let data = field
            .bytes()
            .await
            .map_err(|e| UploadError::WriteError(e.to_string()))?;

        // 检查文件大小
        if data.len() > MAX_FILE_SIZE {
            return Err(UploadError::FileTooLarge);
        }

        // 生成唯一文件名
        let unique_name = format!("{}.{}", Uuid::new_v4(), extension);
        let file_path = upload_dir.join(&unique_name);

        // 写入文件
        fs::write(&file_path, &data)
            .await
            .map_err(|e| UploadError::WriteError(e.to_string()))?;

        // 返回文件 URL
        let url = format!("/uploads/{}", unique_name);
        return Ok(Json(json!({ "url": url })));
    }

    Err(UploadError::NoFile)
}