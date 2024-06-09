use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ClipEntry {
    data: String,
    clipboard_flag: bool,
}

impl ClipEntry {
    fn new<S: Into<String>>(data: S, clipboard_flag: bool) -> Self {
        ClipEntry {
            data: data.into(),
            clipboard_flag,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    entries: [ClipEntry; 3],
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn init_configuration() -> Config {
    let path = confy::get_configuration_file_path("clipmagic", None)
        .map_err(|e| log::error!("{e}"))
        .ok();

    log::info!("retrieve configuration using path: '{path:?}'");

    let config = confy::load::<Config>("clipmagic", None)
        .map_err(|e| log::error!("{e}"))
        .unwrap_or_default();

    log::debug!("using configuration: {config}");

    config
}
