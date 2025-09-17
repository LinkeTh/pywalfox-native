use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct PywalFile {
    colors: Vec<String>,
    wallpaper: Option<String>,
}

fn pywal_colors_path() -> PathBuf {
    let home = directories::BaseDirs::new()
        .expect("xdg base")
        .home_dir()
        .to_path_buf();
    home.join(".cache").join("wal").join("pywalfox.json")
}

pub fn read_pywal() -> Result<(Vec<String>, Option<String>)> {
    let path = pywal_colors_path();
    let data = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let parsed: PywalFile = serde_json::from_str(&data).context("parse pywalfox.json")?;

    // parsed
    //     .colors
    //     .insert("color0".to_string(), "#303235".to_string());
    // let colors: Vec<(String, String)> = parsed.colors.into_iter().collect();
    //
    // let mut colors = colors
    //     .into_iter()
    //     .map(|k| (k.0.replace("color", "").parse::<i32>().unwrap(), k.1))
    //     .collect::<Vec<_>>();
    // colors.sort_by(|a, b| a.0.cmp(&b.0));
    // for color in &colors {
    //     info!("color: {} -> {}", color.0, color.1);
    // }
    // let colors = colors.into_iter().map(|(_, v)| v).collect::<Vec<_>>();

    let colors = parsed.colors;

    if colors.len() < 16 {
        eprint!("pywal colors.json contains fewer than 16 colors");
    }
    Ok((colors, parsed.wallpaper))
}
