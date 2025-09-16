use directories::BaseDirs;
use std::path::PathBuf;

pub const HOST_NAME: &str = "pywalfox"; // keep the same host name used by the Python implementation
pub const ALLOWED_EXTENSION: &str = "pywalfox@frewacom.org"; // Firefox add-on id

pub fn mozilla_native_hosts_dir_user() -> PathBuf {
    // ~/.mozilla/native-messaging-hosts/
    let home = BaseDirs::new()
        .expect("xdg base dirs")
        .home_dir()
        .to_path_buf();
    home.join(".mozilla").join("native-messaging-hosts")
}

pub fn manifest_path_user() -> PathBuf {
    mozilla_native_hosts_dir_user().join(format!("{}.json", HOST_NAME))
}
