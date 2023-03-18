use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use models::auth::Keys;
use postgrest::Postgrest;
use shuttle_secrets::SecretStore;
use sync_wrapper::SyncWrapper;

mod models;
mod routes;

use routes::dashboard::{
    auth::{login, KEYS},
    users::{create_user, get_all_users, update_user},
};

async fn index() -> &'static str {
    // TODO Add documentation here
    return "OSF Database API";
}

pub struct AppState {
    db_client: Postgrest,
}

impl AppState {
    fn new(db_client: Postgrest) -> Self {
        Self { db_client }
    }
}

#[shuttle_service::main]
async fn axum(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleAxum {
    let supabase_url = if cfg!(debug_assertions) {
        secret_store.get("DEV_SUPABASE_URL")
    } else {
        secret_store.get("SUPABASE_URL")
    }
    .expect("Supabase URL not provided");
    let supabase_key = if cfg!(debug_assertions) {
        secret_store.get("DEV_SUPABASE_KEY")
    } else {
        secret_store.get("SUPABASE_KEY")
    }
    .expect("Supabase key not provided");

    let secret = secret_store.get("SECRET").expect("JWT_SECRET must be set");
    match KEYS.set(Keys::new(secret.as_bytes())) {
        Ok(_) => (),
        Err(_) => panic!("Failed to initialize KEYS"),
    }

    let client = Postgrest::new(supabase_url).insert_header("apikey", supabase_key);
    let state = Arc::new(AppState::new(client));

    let router = Router::new()
        .route("/", get(index))
        .route("/api/v1/dashboard/login", post(login))
        .route(
            "/api/v1/dashboard/users",
            get(get_all_users).patch(update_user).post(create_user),
        )
        .with_state(state);

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
