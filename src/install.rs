use crate::config::{
    manifest_path_user, mozilla_native_hosts_dir_user, ALLOWED_EXTENSION, HOST_NAME,
};
use anyhow::{Context, Result};
use std::fs;

#[derive(serde::Serialize)]
struct Manifest<'a> {
    name: &'a str,
    description: &'a str,
    path: String,
    r#type: &'a str,
    allowed_extensions: [&'a str; 1],
}

// fn systemd_user_dir() -> PathBuf {
//     // ~/.config/systemd/user
//     let base = directories::BaseDirs::new().expect("xdg base");
//     base.home_dir().join(".config").join("systemd").join("user")
// }

// fn service_path() -> PathBuf {
//     systemd_user_dir().join("pywalfox.service")
// }

// fn write_systemd_unit(bin: &PathBuf) -> Result<()> {
//     let dir = systemd_user_dir();
//     fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
//     let content = format!(
//         "[Unit]\nDescription=Pywalfox Native Host\nAfter=default.target\n\n[Service]\nExecStart=\"{}\" start\nRestart=on-failure\n\n[Install]\nWantedBy=default.target\n",
//         bin.display()
//     );
//     fs::write(service_path(), content).context("writing systemd unit")?;
//     Ok(())
// }
//
// fn daemon_reload() {
//     let _ = Command::new("systemctl").args(["--user", "daemon-reload"]).status();
// }

pub fn install_manifest(_global: bool) -> Result<()> {
    // Only user-scope for now
    let bin = std::env::current_exe().context("resolve current exe path")?;
    let manifest_dir = mozilla_native_hosts_dir_user();
    fs::create_dir_all(&manifest_dir)
        .with_context(|| format!("creating {}", manifest_dir.display()))?;

    let manifest = Manifest {
        name: HOST_NAME,
        description: "Automatically theme your browser using Pywal colors",
        path: bin.to_string_lossy().to_string(),
        r#type: "stdio",
        allowed_extensions: [ALLOWED_EXTENSION],
    };
    let manifest_path = manifest_path_user();
    let data = serde_json::to_vec_pretty(&manifest)?;
    fs::write(&manifest_path, data)
        .with_context(|| format!("writing manifest {}", manifest_path.display()))?;

    // Systemd user unit
    // write_systemd_unit(&bin)?;
    // daemon_reload();

    println!("Installed manifest at {}", manifest_path.display());
    // println!(
    //     "Installed systemd user unit at {}",
    //     service_path().display()
    // );
    Ok(())
}

pub fn uninstall_manifest(_global: bool) -> Result<()> {
    let path = manifest_path_user();
    if path.exists() {
        fs::remove_file(&path).with_context(|| format!("removing {}", path.display()))?;
        println!("Removed manifest {}", path.display());
    } else {
        println!("Manifest not found at {}", path.display());
    }

    // Remove systemd unit
    // let unit = service_path();
    // if unit.exists() {
    //     fs::remove_file(&unit).with_context(|| format!("removing {}", unit.display()))?;
    // println!("Removed systemd user unit {}", unit.display());
    // daemon_reload();
    // }
    Ok(())
}
