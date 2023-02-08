use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    models::{auth::Claims, error::ApiError},
    AppState,
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UserModel {
    registration_number: u64,
    name: String,
    title: Option<String>,
    phone_number: u64,
    email: String,
    designation: Option<String>,
    department: Option<String>,
    year: u8,
    remarks: Option<String>,
    strikes: u8,
}

pub async fn get_all_users(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<UserModel>>, ApiError> {
    let user_vec = state
        .db_client
        .from("users")
        .select("*")
        .execute()
        .await
        .map_err(|_| ApiError::AuthenticationError)?
        .json::<Vec<UserModel>>()
        .await
        .map_err(|e| {
            println!("{e:?}");
            return ApiError::AuthenticationError;
        })?;

    Ok(Json(user_vec))
}

pub async fn update_user(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserModel>,
) -> Result<String, ApiError> {
    let resp = state
        .db_client
        .from("users")
        .eq(
            "RegistrationNumber",
            payload.registration_number.to_string(),
        )
        .update(serde_json::to_string(&payload).map_err(|e| {
            println!("1 {e:?}");
            ApiError::AuthenticationError
        })?)
        .execute()
        .await
        .map_err(|e| {
            println!("2 {e:?}");
            ApiError::AuthenticationError
        })?;

    let str = resp.json::<Vec<UserModel>>().await.map_err(|e| {
        println!("3 {e:?}");
        ApiError::AuthenticationError
    })?;

    println!("{str:?}");

    Ok("yus".to_string())
}

pub async fn create_user(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserModel>,
) -> Result<String, ApiError> {
    Ok("".to_string())
}
