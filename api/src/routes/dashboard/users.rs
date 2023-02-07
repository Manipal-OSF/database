use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    models::{auth::User, error::ApiError},
    AppState,
};

use super::auth::Claims;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UserModel {
    name: String,
    registration_number: u64,
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
