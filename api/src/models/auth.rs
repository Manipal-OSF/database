use axum::extract::FromRequestParts;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt, TypedHeader};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::routes::dashboard::auth::KEYS;

use super::error::ApiError;

#[derive(Deserialize)]
pub struct AuthModel {
    pub hash: String,
    pub salt: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginPayload {
    pub api_key: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub exp: u32,
}

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
        let token_data = decode::<Claims>(
            bearer.token(),
            &KEYS.get().unwrap().decoding,
            &Validation::default(),
        )
        .map_err(|_| ApiError::AuthenticationError)?;

        Ok(token_data.claims)
    }
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
