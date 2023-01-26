use std::sync::Arc;

use axum::{routing::get, Router};
use postgrest::Postgrest;
use shuttle_secrets::SecretStore;
use sync_wrapper::SyncWrapper;

mod routes;

use routes::dashboard::auth::login;

async fn index() -> &'static str {
    // TODO Add documentation here
    return "OSF Database API";
}

struct State {
    db_client: Postgrest,
}

#[shuttle_service::main]
async fn axum(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleAxum {
    let supabase_url = match secret_store.get("SUPABASE_URL") {
        Some(secret) => secret,
        None => panic!("Supabase URL not provided!"),
    };

    let client = Postgrest::new(supabase_url);
    let state = Arc::new(State { db_client: client });

    let router = Router::new()
        .route("/", get(index))
        .route("/api/v1/dashboard/login", get(login))
        .with_state(state);

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
