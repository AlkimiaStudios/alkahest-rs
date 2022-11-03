#[cfg(unix)]
const CONFIG_LOCATION: &str = "$HOME/.config/alkahest/config.toml";
#[cfg(windows)]
const CONFIG_LOCATION: &str = "%PROGRAMDATA%\\AppData\\Local\\Alkahest\\config.toml";

use super::resources::{AssetManager, ConfigManager, ConfigContext};

pub(crate) fn init() -> ConfigContext {
    let parsed_path = CONFIG_LOCATION.replace("$HOME", &std::env::var("HOME").unwrap_or(String::from("")));
    ConfigManager::load_direct(String::from(parsed_path))
}
