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
    // * GET /api/v1/dashboard/users

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
) -> Result<Json<Vec<UserModel>>, ApiError> {
    // * PATCH /api/v1/dashboard/users

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

    let model = resp.json::<Vec<UserModel>>().await.map_err(|e| {
        println!("3 {e:?}");
        ApiError::AuthenticationError
    })?;

    println!("{model:?}");

    Ok(Json(model))
}

pub async fn create_user(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserModel>,
) -> Result<Json<Vec<UserModel>>, ApiError> {
    // * POST /api/v1/dashboard/users

    let resp = state
        .db_client
        .from("users")
        .insert(serde_json::to_string(&payload).map_err(|e| ApiError::AuthenticationError)?)
        .execute()
        .await
        .map_err(|e| ApiError::AuthenticationError)?;

    let model = resp.json::<Vec<UserModel>>().await.map_err(|e| {
        println!("3 {e:?}");
        ApiError::AuthenticationError
    })?;

    println!("{model:?}");

    Ok(Json(model))
}
