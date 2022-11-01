use super::{AssetManager, AssetHandle, Asset};
use crate::util::containers::{ContainerError, HandleMap};

#[derive(Clone, Debug)]
pub(crate) struct WindowConfig {
    title: String,
    width: u32,
    height: u32,
    hint: String,
}

#[derive(Clone, Debug)]
pub(crate) struct Config {
    window: WindowConfig,
}

impl Asset for Config {}

pub(crate) struct ConfigManager {
    cache: HandleMap<Config>,
}

impl AssetManager<Config> for ConfigManager {
    fn init(cache_size: usize) -> ConfigManager {
        ConfigManager {
            cache: HandleMap::new(0, cache_size)
        }
    }

    //TODO: Maybe return Option<T> from the loading functions???
    fn load_to_cache(&mut self, _path: String) -> AssetHandle {
        let config: Config;

        //TODO: load config from file

        self.cache.insert(config)
    }

    fn load_direct(_path: String) -> Config {
        let config: Config;

        //TODO: load config from file

        config
    }

    fn load_from_cache(&self, handle: AssetHandle) -> Result<Config, ContainerError> {
        self.cache.get(handle)
    }

    fn purge_from_cache(&mut self, handle: AssetHandle) -> Result<(), ContainerError> {
        self.cache.erase(handle)
    }
}
