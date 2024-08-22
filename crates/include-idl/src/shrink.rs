use std::io::Write;
use std::path::Path;
use std::{fs, io};

use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read idl")]
    Read(#[source] io::Error),
    #[error("invalid idl json")]
    Json(#[source] serde_json::Error),
    #[error("failed to compress idl")]
    Compress(#[source] io::Error),
    #[error("failed to write compressed idl")]
    Write(#[source] io::Error),
}

pub fn compress_idl(idl_src: &Path, idl_dst: &Path) -> Result<(), Error> {
    let json = fs::read(idl_src).map_err(Error::Read)?;

    let idl = serde_json::from_slice::<Value>(&json).map_err(Error::Json)?;
    let json = serde_json::to_string(&idl).map_err(Error::Json)?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(json.as_bytes()).map_err(Error::Compress)?;
    let compressed = encoder.finish().map_err(Error::Compress)?;

    fs::write(idl_dst, &compressed).map_err(Error::Write)?;

    Ok(())
}
