use std::sync::Arc;

use axum::{extract::State, Json};
use base64::{engine::general_purpose, Engine};
use jsonwebtoken::{encode, Header};
use once_cell::sync::Lazy;
use sha2::{Digest, Sha512};

use crate::models::auth::{AuthBody, AuthModel, Claims, Keys, LoginPayload};
use crate::models::error::ApiError;
use crate::AppState;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub async fn login(
    State(state): State<Arc<AppState>>,
    payload: Json<LoginPayload>,
) -> Result<Json<AuthBody>, ApiError> {
    // * POST /api/v1/dashboard/login

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

    verify(json.hash.as_str(), json.salt.as_str(), &payload.api_key)?;

    let claims = Claims { exp: 2000000000 };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| ApiError::AuthenticationError)?;

    Ok(Json(AuthBody::new(token)))
}

pub fn verify<'a>(hash: &'a str, salt: &'a str, password: &'a str) -> Result<(), ApiError> {
    let mut hasher = Sha512::new();
    hasher.update(format!("{password}{salt}"));
    let result = hasher.finalize();

    let user_hash = general_purpose::STANDARD.encode(result);

    if user_hash == hash {
        Ok(())
    } else {
        Err(ApiError::AuthenticationError)
    }
}
