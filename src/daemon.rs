use crate::native_messaging::{
    read_message, send_colors, send_invalid_response, send_theme_mode, send_version, Request,
};
use std::io::Write;

use anyhow::Result;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;
use tracing::{error, info};
/*
ACTIONS = {
    'VERSION': 'debug:version',
    'OUTPUT': 'debug:output',
    'COLORS': 'action:colors',
    'INVALID_ACTION': 'action:invalid',
    'CSS_ENABLE': 'css:enable',
    'CSS_DISABLE': 'css:disable',
    'CSS_FONT_SIZE': 'css:font:size',
    'THEME_MODE': 'theme:mode',
}
 */

const SOCKET_PATH: &str = "/tmp/mypy.sock";
pub fn run() -> Result<()> {
    info!("starting pywalfox daemon");
    start_socket_server()?;
    while let Some(msg) = read_message::<Request>()? {
        match msg.action.as_str() {
            "debug:version" => {
                send_version()?;
            }
            "action:colors" => {
                send_colors()?;
            }
            _ => {
                send_invalid_response()?;
            }
        }
    }

    Ok(())
}

fn handle_client(stream: UnixStream) {
    let reader = BufReader::new(&stream);
    for line in reader.lines() {
        match line {
            Ok(cmd) => {
                info!("Received command: {}", cmd);
                if cmd == "update" {
                    send_colors().unwrap();
                } else if cmd == "auto" {
                    send_theme_mode("auto").unwrap();
                } else if cmd == "dark" {
                    send_theme_mode("dark").unwrap();
                } else if cmd == "light" {
                    send_theme_mode("light").unwrap();
                }
            }
            Err(e) => {
                error!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

pub(crate) fn send_command(cmd: &str) -> Result<()> {
    let mut stream = UnixStream::connect(SOCKET_PATH)?;
    writeln!(stream, "{}", cmd)?;
    Ok(())
}
fn start_socket_server() -> Result<()> {
    let _ = std::fs::remove_file(SOCKET_PATH); // clean up stale socket
    let listener = UnixListener::bind(SOCKET_PATH)?;
    info!("Server started, listening on {}", SOCKET_PATH);

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| handle_client(stream));
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }
    });
    Ok(())
}
