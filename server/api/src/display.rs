use anyhow::Result;
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use dto::DisplayState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{error::RestError, AppState};

#[derive(Serialize, Deserialize)]
struct DisplayStatePayload {
    state: DisplayState,
}

#[derive(Serialize, Deserialize)]
struct BrightnessPayload {
    brightness: u8,
}

async fn set_state(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DisplayStatePayload>,
) -> Result<(), RestError> {
    service::display::set_state(payload.state, &state.channel).map_err(|e| e.into())
}

async fn get_state() -> Result<Json<DisplayStatePayload>, RestError> {
    let state = service::display::get_state()?;
    Ok(Json(DisplayStatePayload { state }))
}

async fn set_brightness(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BrightnessPayload>,
) -> Result<(), RestError> {
    service::display::set_brightness(payload.brightness, &state.channel);
    Ok(())
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/state", get(get_state).post(set_state))
        .route("/brightness", post(set_brightness))
        .with_state(state)
}
