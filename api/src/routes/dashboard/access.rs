use std::sync::Arc;

use axum::extract::State;

use crate::{models::error::ApiError, AppState};

use super::auth::Claims;

pub async fn get_all_users(
    claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<String, ApiError> {
    Ok(format!("{claims:?}"))
}
