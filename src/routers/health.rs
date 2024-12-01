use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::helpers::utils::get_version;

#[derive(Serialize, Deserialize)]
struct VersionResponse {
    version: String,
}

pub async fn health() -> impl IntoResponse {
    if let Ok(version) = get_version().await {
        (StatusCode::OK, Json(VersionResponse { version })).into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "version file not found").into_response()
    }
}
