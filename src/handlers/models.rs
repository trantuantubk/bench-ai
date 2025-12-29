use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use crate::models::Model;

#[derive(Debug, Deserialize)]
pub struct CreateModelRequest {
    name: String,
    framework: String,
    version: String,
}

#[derive(Serialize, Debug)]
pub struct CreateModelResponse {
    id: String,
    name: String,
    framework: String,
    version: String,
}

pub async fn create_model(
    Json(payload): Json<CreateModelRequest>,
) -> Result<(StatusCode, Json<CreateModelResponse>), (StatusCode, String)> {
    // Validate and create model
    let model = Model::new(payload.name, payload.framework, payload.version)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    // Return created model
    let response = CreateModelResponse {
        id: model.id().to_string(),
        name: model.name().to_string(),
        framework: model.framework().to_string(),
        version: model.version().to_string(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}
