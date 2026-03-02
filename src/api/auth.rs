use crate::api::AppState;
use crate::models::{LoginRequest, TotpVerifyRequest};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct TotpEnableRequestFull {
    pub secret: String,
    pub backup_codes: Vec<String>,
    pub code: String,
}

/// 第一步登录：验证用户名密码
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match state.auth_service.login_step_one(&request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((StatusCode::UNAUTHORIZED, e.to_string())),
    }
}

/// 第二步登录：验证TOTP码
pub async fn verify_totp(
    State(state): State<Arc<AppState>>,
    Json(request): Json<TotpVerifyRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 验证session token
    let username = match state.auth_service.verify_session_token(&request.session_token) {
        Ok(username) => username,
        Err(e) => return Err((StatusCode::UNAUTHORIZED, e.to_string())),
    };

    // 验证TOTP码
    match state.totp_service.verify_code(&request.code).await {
        Ok(true) => {
            // 生成JWT token
            match state.auth_service.login_step_two(&username) {
                Ok(response) => Ok(Json(response)),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
        Ok(false) => Err((StatusCode::UNAUTHORIZED, "Invalid verification code".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// 获取TOTP状态
pub async fn get_totp_status(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match state.totp_service.is_enabled().await {
        Ok(enabled) => Ok(Json(serde_json::json!({ "enabled": enabled }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// 初始化TOTP设置
pub async fn setup_totp(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 从环境变量获取用户名
    let username = std::env::var("AUTH_USERNAME").unwrap_or_else(|_| "admin".to_string());

    match state.totp_service.generate_setup(&username).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// 启用TOTP
pub async fn enable_totp(
    State(state): State<Arc<AppState>>,
    Json(request): Json<TotpEnableRequestFull>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match state.totp_service.enable_totp(&request.secret, &request.backup_codes, &request.code).await {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

/// 禁用TOTP
pub async fn disable_totp(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match state.totp_service.disable_totp().await {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// 重新生成备用码
pub async fn regenerate_backup_codes(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match state.totp_service.regenerate_backup_codes().await {
        Ok(codes) => Ok(Json(serde_json::json!({ "backup_codes": codes }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
