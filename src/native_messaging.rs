use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::{self, Read, Write};

/// Read one native messaging JSON message from stdin.
pub fn read_message<T: DeserializeOwned + std::fmt::Debug>() -> Result<Option<T>> {
    // info!("reading native message");
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
    // info!("received data: {:?}", data);
    let value = serde_json::from_slice::<T>(&data).context("parsing json")?;
    // info!("received message: {:?}", value);
    Ok(Some(value))
}

/// Write one native messaging JSON message to stdout.
pub fn write_message<T: Serialize>(value: &T) -> Result<()> {
    let data = serde_json::to_vec(value).context("serialize json")?;
    // info!("sending data: {data} i dont get it ");
    let len = data.len() as u32;
    let mut out = io::stdout();
    out.write_all(&len.to_le_bytes()).context("write len")?;
    out.write_all(&data).context("write body")?;
    out.flush().ok();
    Ok(())
}
