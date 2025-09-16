use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct PywalFile {
    colors: BTreeMap<String, String>,
    wallpaper: Option<String>,
}

fn pywal_colors_path() -> PathBuf {
    let home = directories::BaseDirs::new()
        .expect("xdg base")
        .home_dir()
        .to_path_buf();
    home.join(".cache").join("wal").join("colors.json")
}

pub fn read_pywal() -> Result<(Vec<String>, Option<String>)> {
    let path = pywal_colors_path();
    let data = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let parsed: PywalFile = serde_json::from_str(&data).context("parse colors.json")?;
    let mut colors: Vec<(String, String)> = parsed.colors.into_iter().collect();
    colors.sort_by(|a, b| a.0.cmp(&b.0));
    let colors = colors.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
    if colors.len() < 16 {
        eprint!("pywal colors.json contains fewer than 16 colors");
    }
    Ok((colors, parsed.wallpaper))
}

pub fn handle_update() -> Result<()> {
    let (colors, wallpaper) = read_pywal()?;
    Ok(())
}

pub fn handle_dark() -> Result<()> {
    Ok(())
}

pub fn handle_light() -> Result<()> {
    Ok(())
}

pub fn handle_auto() -> Result<()> {
    Ok(())
}
