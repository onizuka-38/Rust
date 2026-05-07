use crate::analysis::analyze_incident;
use crate::llm::LlmClient;
use crate::model::{IncidentInput, IncidentReport};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct AppState {
    llm: Arc<dyn LlmClient>,
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

#[derive(Serialize)]
struct ApiError {
    message: String,
}

pub fn router(llm: Arc<dyn LlmClient>) -> Router {
    let state = AppState { llm };

    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/incidents/analyze", post(analyze))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn healthz() -> Json<Health> {
    Json(Health { status: "ok" })
}

async fn analyze(
    State(state): State<AppState>,
    Json(input): Json<IncidentInput>,
) -> Result<Json<IncidentReport>, (StatusCode, Json<ApiError>)> {
    input.validate().map_err(|message| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiError { message }),
        )
    })?;

    analyze_incident(input, state.llm.as_ref())
        .await
        .map(Json)
        .map_err(|err| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ApiError {
                    message: err.to_string(),
                }),
            )
        })
}
