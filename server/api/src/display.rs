use anyhow::Result;
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use dto::DisplayState;
use std::sync::Arc;

use crate::{error::RestError, AppState};

async fn set_state(
    Path(action): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<(), RestError> {
    let display_state = match action.as_str() {
        "on" => DisplayState::On,
        "off" => DisplayState::Off,
        _ => {
            return Err(RestError::Invalid(
                "invalid action: expected on or off".to_string(),
            ))
        }
    };
    service::display::set_state(display_state, &state.channel).map_err(|e| e.into())
}

async fn get_state() -> Result<String, RestError> {
    service::display::get_state().map_err(|e| e.into())
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/state/:action", post(set_state))
        .route("/state", get(get_state))
        .with_state(state)
}
