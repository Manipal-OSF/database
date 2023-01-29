use std::sync::Arc;

use axum::{extract::State, Json};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

use crate::models::error::ApiError;
use crate::AppState;

#[derive(Deserialize)]
pub struct AuthModel {
    hash: String,
    salt: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    api_key: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    payload: Json<LoginRequest>,
) -> Result<&'static str, ApiError> {
    // * /api/v1/dashboard/login

    let json = &state
        .db_client
        .from("auth")
        .eq("id", "dashboard")
        .select("*")
        .single()
        .execute()
        .await
        .map_err(|_| ApiError::AuthenticationError)?
        .json::<AuthModel>()
        .await
        .map_err(|_| ApiError::AuthenticationError)?;

    verify(json.hash.as_str(), json.salt.as_str(), &payload.api_key)
}

pub fn verify<'a>(
    hash: &'a str,
    salt: &'a str,
    password: &'a str,
) -> Result<&'static str, ApiError> {
    let mut hasher = Sha512::new();
    hasher.update(format!("{password}{salt}"));
    let result = hasher.finalize();

    let user_hash = general_purpose::STANDARD.encode(result);

    if user_hash == hash {
        Ok("Success")
    } else {
        Err(ApiError::AuthenticationError)
    }
}
