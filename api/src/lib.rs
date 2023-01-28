use std::sync::Arc;

use axum::{routing::get, Router};
use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer},
    memory_store::MemoryStore as AuthMemoryStore,
    AuthLayer, UserStore,
};
use postgrest::Postgrest;
use shuttle_secrets::SecretStore;
use sync_wrapper::SyncWrapper;

mod models;
mod routes;

use routes::dashboard::auth::login;

async fn index() -> &'static str {
    // TODO Add documentation here
    return "OSF Database API";
}

pub struct AppState {
    db_client: Postgrest,
}

#[shuttle_service::main]
async fn axum(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleAxum {
    let supabase_url = secret_store
        .get("SUPABASE_URL")
        .expect("Supabase URL not provided!");
    let supabase_key = secret_store
        .get("SUPABASE_KEY")
        .expect("Supabase key not provided!");

    let secret = "hello";

    let session_store = SessionMemoryStore::new();
    let session_layer = SessionLayer::new(session_store, secret.as_bytes());

    let client = Postgrest::new(supabase_url).insert_header("apikey", supabase_key);
    let state = Arc::new(AppState { db_client: client });

    let router = Router::new()
        .route("/", get(index))
        .route("/api/v1/dashboard/login", get(login))
        .route("/api/v1/dashboard/logout", get(login))
        .with_state(state);

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
