use anyhow::{anyhow, Result};
use log::error;
use std::{env, process::Command};

use dto::{DisplayState, Message};
use tokio::sync::broadcast::Sender;

pub fn set_state(display_state: DisplayState, channel: &Sender<Message>) -> Result<()> {
    let display_cmd = env::var("DISPLAY_CMD").unwrap_or("vcgencmd".to_string());
    let on_args = env::var("DISPLAY_ON_ARGS").unwrap_or("display_power 1".to_string());
    let off_args = env::var("DISPLAY_OFF_ARGS").unwrap_or("display_power 0".to_string());
    let args: Vec<&str> = match display_state {
        DisplayState::On => on_args.split(" ").collect(),
        DisplayState::Off => off_args.split(" ").collect(),
    };
    let output = Command::new(display_cmd)
        .args(args)
        .output()
        .map_err(|e| anyhow!(e))?;
    if !output.status.success() {
        let code = output.status.code().unwrap_or(-1);
        let output = output.status.to_string();
        error!("unexpected exit {code}: {output}");
        return Err(anyhow!("unexpected exit code"));
    }
    if let Err(e) = channel.send(Message::DisplayStateChange(display_state)) {
        error!("failed to broadcast state change: {e}");
    }
    Ok(())
}

pub fn get_state() -> Result<String> {
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
        return Err(anyhow!("unexpected exit code {code}: {output}"));
    }
    let stdout = String::from_utf8(output.stdout).map_err(|e| anyhow!(e))?;
    Ok(stdout)
}
