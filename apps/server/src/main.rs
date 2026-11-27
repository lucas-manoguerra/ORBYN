use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use tower_http::trace::TraceLayer;

async fn health() -> Json<Value> {
    Json(json!({ "servicio": "orbyn-control-plane", "estado": "disponible" }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http());

    let address = "127.0.0.1:8787";
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(%address, "Plano de control ORBYN iniciado");
    axum::serve(listener, app).await?;
    Ok(())
}
