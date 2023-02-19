use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    models::{auth::Claims, error::ApiError},
    AppState,
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
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
        .map_err(|err| ApiError::ServerError(err.to_string()))?
        .json::<Vec<UserModel>>()
        .await
        .map_err(|err| ApiError::ServerError(err.to_string()))?;

    Ok(Json(user_vec))
}

fn validate(user: &UserModel) -> Result<(), ApiError> {
    if !user.name.is_ascii() {
        return Err(ApiError::ValidationError(
            "Invalid name entered".to_string(),
        ));
    } else if user.name.trim().is_empty() {
        return Err(ApiError::ValidationError(
            "Name cannot be empty".to_string(),
        ));
    }

    if let Some(title) = &user.title {
        if !["Founder", "CoFounder"].contains(&title.trim()) {
            return Err(ApiError::ValidationError(
                "Title has to be one of Founder or CoFounder".to_string(),
            ));
        }
    }

    if user.phone_number > 9999999999 {
        return Err(ApiError::ValidationError(
            "Phone number cannot be greater than 10 digits long".to_string(),
        ));
    }

    // Email regex following Google RFC2822
    // Yes, I know its insane
    if user.email.matches(r"[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?").count() == 0 {
        return Err(ApiError::ValidationError(
            "Invalid email entered".to_string(),
        ));
    }

    if let Some(title) = &user.designation {
        if !["Board", "Interim", "F1", "F2", "WC", "MC"].contains(&title.trim()) {
            return Err(ApiError::ValidationError(
                "Invalid designation entered".to_string(),
            ));
        }
    }

    if let Some(title) = &user.department {
        if ![
            "Academics",
            "Development",
            "Relations",
            "Multimedia",
            "CreatorCollaboratorProgram",
        ]
        .contains(&title.trim())
        {
            return Err(ApiError::ValidationError(
                "Invalid department entered".to_string(),
            ));
        }
    }

    if user.year < 1 || user.year > 4 {
        return Err(ApiError::ValidationError(
            "Year can only be one of 1, 2, 3 and 4".to_string(),
        ));
    }

    Ok(())
}

pub async fn update_user(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserModel>,
) -> Result<Json<Vec<UserModel>>, ApiError> {
    // * PATCH /api/v1/dashboard/users

    validate(&payload)?;

    let resp = state
        .db_client
        .from("users")
        .eq(
            "registrationNumber",
            payload.registration_number.to_string(),
        )
        .update(
            serde_json::to_string(&payload)
                .map_err(|err| ApiError::ServerError(err.to_string()))?,
        )
        .execute()
        .await
        .map_err(|err| ApiError::ServerError(err.to_string()))?;

    let model = resp
        .json::<Vec<UserModel>>()
        .await
        .map_err(|err| ApiError::ServerError(err.to_string()))?;

    Ok(Json(model))
}

pub async fn create_user(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserModel>,
) -> Result<Json<Vec<UserModel>>, ApiError> {
    // * POST /api/v1/dashboard/users

    validate(&payload)?;

    let resp = state
        .db_client
        .from("users")
        .insert(
            serde_json::to_string(&payload)
                .map_err(|err| ApiError::ServerError(err.to_string()))?,
        )
        .execute()
        .await
        .map_err(|err| ApiError::ServerError(err.to_string()))?;

    let model = resp
        .json::<Vec<UserModel>>()
        .await
        .map_err(|err| ApiError::ServerError(err.to_string()))?;

    Ok(Json(model))
}
