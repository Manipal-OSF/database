use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt, TypedHeader};
use axum::{extract::State, Json};
use base64::{engine::general_purpose, Engine};
use jsonwebtoken::{decode, encode, Header, Validation};
use once_cell::sync::Lazy;
use sha2::{Digest, Sha512};

use crate::models::auth::{AuthBody, AuthModel, Claims, Keys, LoginPayload};
use crate::models::error::ApiError;
use crate::AppState;

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ApiError::AuthenticationError)?;

        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| ApiError::AuthenticationError)?;

        Ok(token_data.claims)
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub async fn login(
    State(state): State<Arc<AppState>>,
    payload: Json<LoginPayload>,
) -> Result<Json<AuthBody>, ApiError> {
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
