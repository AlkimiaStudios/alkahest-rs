use super::{AssetManager, AssetHandle, Asset};
use crate::util::containers::{ContainerError, HandleMap};
use serde_derive::Deserialize;
use std::fs;
use toml;
use crate::trace;
use std::collections::HashSet;
use string_interner::{StringInterner, DefaultSymbol};

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub hint: String,
}
impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig {
            title: String::from("Alkahest"),
            width: 1920,
            height: 1080,
            hint: String::from("alkahest")
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct ConfigContext {
    pub window: WindowConfig,
}

impl Asset for ConfigContext {}

pub(crate) struct ConfigManager {
    cache: HandleMap<ConfigContext>,
    file_map: HashSet<DefaultSymbol>,
}

impl AssetManager<ConfigContext> for ConfigManager {
    fn init(cache_size: usize) -> ConfigManager {
        ConfigManager {
            cache: HandleMap::new(0, cache_size),
            file_map: HashSet::new(),
        }
    }

    //TODO: Maybe return Option<T> from the loading functions???
    fn load_to_cache(&mut self, path: String) -> AssetHandle {
        let mut interner = StringInterner::default();
        if self.file_map.contains(&interner.get_or_intern(&path)) {
            //TODO: return the AssetHandle of the cached asset
        }

        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => String::from(""),
        };

        let config: ConfigContext = match toml::from_str(&contents) {
            Ok(c) => c,
            Err(_) => ConfigContext::default(),
        };

        self.cache.insert(config)
    }

    fn load_direct(path: String) -> ConfigContext {
        trace!("Loading config file from: {}", path);
        let p = std::path::PathBuf::from(path);
        let contents = match fs::read_to_string(p) {
            Ok(c) => c,
            Err(_) => String::from(""),
        };

        let config: ConfigContext = match toml::from_str(&contents) {
            Ok(c) => c,
            Err(_) => ConfigContext::default(),
        };

        config
    }

    fn load_from_cache(&self, handle: AssetHandle) -> Result<ConfigContext, ContainerError> {
        self.cache.get(handle)
    }

    fn purge_from_cache(&mut self, handle: AssetHandle) -> Result<(), ContainerError> {
        self.cache.erase(handle)
    }
}
