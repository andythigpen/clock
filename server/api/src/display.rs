use anyhow::{anyhow, Result};
use axum::{
    extract::Path,
    routing::{get, post},
    Router,
};
use log::error;
use std::{env, process::Command};

use crate::error::RestError;

async fn set_state(Path(state): Path<String>) -> Result<(), RestError> {
    let display_cmd = env::var("DISPLAY_CMD").unwrap_or("vcgencmd".to_string());
    let on_args = env::var("DISPLAY_ON_ARGS").unwrap_or("display_power 1".to_string());
    let off_args = env::var("DISPLAY_OFF_ARGS").unwrap_or("display_power 0".to_string());
    let args: Vec<&str> = match state.as_str() {
        "on" => on_args.split(" ").collect(),
        "off" => off_args.split(" ").collect(),
        _ => {
            return Err(RestError::Invalid(
                "invalid state: expected on or off".to_string(),
            ))
        }
    };
    let output = Command::new(display_cmd)
        .args(args)
        .output()
        .map_err(|e| anyhow!(e))?;
    if !output.status.success() {
        let code = output.status.code().unwrap_or(-1);
        let output = output.status.to_string();
        error!("unexpected exit {code}: {output}");
        return Err(RestError::Internal(anyhow!("unexpected exit code")));
    }
    Ok(())
}

async fn get_state() -> Result<String, RestError> {
    let display_cmd = env::var("DISPLAY_CMD").unwrap_or("vcgencmd".to_string());
    let args: Vec<String> = env::var("DISPLAY_GET_ARGS")
        .unwrap_or("display_power".to_string())
        .split(" ")
        .map(|a| a.to_string())
        .collect();
    let output = Command::new(display_cmd)
        .args(args)
        .output()
        .map_err(|e| anyhow!(e))?;
    if !output.status.success() {
        let code = output.status.code().unwrap_or(-1);
        let output = output.status.to_string();
        error!("unexpected exit {code}: {output}");
        return Err(RestError::Internal(anyhow!("unexpected exit code")));
    }
    let stdout = String::from_utf8(output.stdout).map_err(|e| anyhow!(e))?;
    Ok(stdout)
}

pub fn router() -> Router {
    Router::new()
        .route("/state/:state", post(set_state))
        .route("/state", get(get_state))
}
