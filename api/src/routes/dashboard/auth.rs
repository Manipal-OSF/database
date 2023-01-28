use std::sync::Arc;

use axum::extract::State;

use crate::AppState;

pub async fn login(State(state): State<Arc<AppState>>) -> &'static str {
    // * /api/v1/dashboard/auth/login

    "Login route"
}

pub fn verify(hash: String) {}
