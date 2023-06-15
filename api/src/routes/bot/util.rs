use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{models::error::ApiError, AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    result: bool,
}

pub async fn validate_discord_id(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Payload>,
) -> Result<Json<Response>, ApiError> {
    let result = state
        .db_client
        .from("users")
        .eq("discord", payload.id.to_string())
        .select("discord")
        .execute()
        .await
        .map_err(|_| {
            ApiError::ServerError(String::from(
                "Unable to fetch & validate data from the server",
            ))
        })?;

    println!(
        "{}",
        result
            .text()
            .await
            .map_err(|_| ApiError::AuthenticationError)?
    );

    Ok(Json(Response { result: true }))
}
