use crate::themes;
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use tracing::info;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct Request {
    pub action: String,
    target: Option<String>,
    size: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct Response<T> {
    action: String,
    success: bool,
    error: Option<String>,
    data: Option<T>, // Error { message: String },
}
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct ColorData {
    colors: Vec<String>,
    wallpaper: Option<String>,
}
/// Read one native messaging JSON message from stdin.
pub fn read_message<T: DeserializeOwned + std::fmt::Debug>() -> Result<Option<T>> {
    let mut len_buf = [0u8; 4];
    if io::stdin().read_exact(&mut len_buf).is_err() {
        eprint!("error reading native message length:");
        // EOF or no more input
        return Ok(None);
    }
    let len = u32::from_le_bytes(len_buf) as usize;
    let mut data = vec![0u8; len];
    io::stdin()
        .read_exact(&mut data)
        .context("reading native message body")?;
    let value = serde_json::from_slice::<T>(&data).context("parsing json")?;
    Ok(Some(value))
}

/// Write one native messaging JSON message to stdout.
pub fn write_message<T: Serialize>(value: &T) -> Result<()> {
    let data = serde_json::to_vec(value).context("serialize json")?;
    let len = data.len() as u32;
    let mut out = io::stdout();
    out.write_all(&len.to_le_bytes()).context("write len")?;
    out.write_all(&data).context("write body")?;
    out.flush().ok();
    Ok(())
}

pub fn send_version() -> Result<()> {
    let response = Response {
        action: "debug:version".to_string(),
        success: true,
        error: None,
        data: Some("2.7.4".to_string()),
    };
    info!("sending =>  {:?}", response);
    write_message(&response)
}

pub fn send_colors() -> Result<()> {
    let colors = themes::read_pywal().expect("Could not read colors from pywal");
    let response = Response {
        action: "action:colors".to_string(),
        success: true,
        error: None,
        data: Some(ColorData {
            colors: colors.0,
            wallpaper: colors.1,
        }),
    };
    info!("sending =>  {:?}", response);
    write_message(&response)
}

pub fn send_theme_mode(mode: &str) -> Result<()> {
    let response = Response {
        action: "theme:mode".to_string(),
        success: true,
        error: None,
        data: Some(mode),
    };
    info!("sending =>  {:?}", response);
    write_message(&response)
}
pub fn send_invalid_response() -> Result<()> {
    let response = build_invalid_response();
    info!("sending =>  {:?}", response);
    write_message(&response)
}

fn build_invalid_response() -> Response<String> {
    Response {
        action: "action:invalid".to_string(),
        success: false,
        error: None,
        data: Some("Invalid action".to_string()),
    }
}
