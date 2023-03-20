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
    year: u16, // Year of joining college
    remarks: Option<String>,
    strikes: u8,
    discord: Option<u64>,
    github: Option<String>,
    location: String,
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
    // Per field validation
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

    if user.year < 2000 {
        return Err(ApiError::ValidationError(
            "Year should be the year of joining the college.".to_string(),
        ));
    }

    if !["Manipal", "Bangalore"].contains(&user.location.as_str().trim()) {
        return Err(ApiError::ValidationError(
            "Invalid location entered. Location should be either of Manipal or Bangalore."
                .to_string(),
        ));
    }

    // Related fields validation
    if user.department == Some("Development".to_string())
        && (user.discord.is_none() || user.github.is_none())
    {
        return Err(ApiError::ValidationError(
            "Discord and GitHub ID cannot be null if the user is in the Development department."
                .to_string(),
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
