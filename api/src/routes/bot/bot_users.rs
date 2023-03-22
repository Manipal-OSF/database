use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    models::{auth::Claims, error::ApiError},
    AppState,
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BotUserModel {
    id: u64,
    discriminator: String,
    username: String,
    avatar: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IDPayload {
    id: u64,
}

pub async fn get_all_bot_users(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<BotUserModel>>, ApiError> {
    // * GET /api/v1/bot/users

    let user_vec = state
        .db_client
        .from("bot")
        .select("*")
        .execute()
        .await
        .map_err(|err| ApiError::ServerError(err.to_string()))?
        .json::<Vec<BotUserModel>>()
        .await
        .map_err(|err| ApiError::ServerError(err.to_string()))?;

    Ok(Json(user_vec))
}

fn validate(user: &BotUserModel) -> Result<(), ApiError> {
    // Per field validation

    // Related fields validation

    Ok(())
}

pub async fn update_bot_user(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BotUserModel>,
) -> Result<Json<Vec<BotUserModel>>, ApiError> {
    // * PATCH /api/v1/bot/users

    validate(&payload)?;

    let resp = state
        .db_client
        .from("bot")
        .eq("id", payload.id.to_string())
        .update(
            serde_json::to_string(&payload)
                .map_err(|err| ApiError::ServerError(err.to_string()))?,
        )
        .execute()
        .await
        .map_err(|err| ApiError::ServerError(err.to_string()))?;

    let model = resp
        .json::<Vec<BotUserModel>>()
        .await
        .map_err(|err| ApiError::ServerError(err.to_string()))?;

    Ok(Json(model))
}

pub async fn create_bot_user(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BotUserModel>,
) -> Result<Json<Vec<BotUserModel>>, ApiError> {
    // * POST /api/v1/bot/users

    validate(&payload)?;

    let resp = state
        .db_client
        .from("bot")
        .insert(
            serde_json::to_string(&payload)
                .map_err(|err| ApiError::ServerError(err.to_string()))?,
        )
        .execute()
        .await
        .map_err(|err| ApiError::ServerError(err.to_string()))?;

    let model = resp
        .json::<Vec<BotUserModel>>()
        .await
        .map_err(|err| ApiError::ServerError(err.to_string()))?;

    Ok(Json(model))
}

pub async fn delete_bot_user(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<IDPayload>,
) -> Result<String, ApiError> {
    // * DELETE /api/v1/bot/users

    let resp = state
        .db_client
        .from("bot")
        .eq("id", payload.id.to_string())
        .delete()
        .execute()
        .await
        .map_err(|err| ApiError::ServerError(err.to_string()))?;

    println!("{resp:?}");

    // let model = resp
    //     .json::<Vec<BotUserModel>>()
    //     .await
    //     .map_err(|err| ApiError::ServerError(err.to_string()))?;

    return Ok("Success".to_string());
}
