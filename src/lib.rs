use axum::{routing::get, Router};
use sync_wrapper::SyncWrapper;

async fn home() -> &'static str {
    "welcome"
}

async fn pong() -> &'static str {
    "ping"
}

#[shuttle_service::main]
async fn axum() -> Result<SyncWrapper<Router>, shuttle_service::Error> {
    let router = Router::new()
        .route("/", get(home))
        .route("/ping", get(pong));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
