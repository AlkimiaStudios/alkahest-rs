use super::{AssetManager, AssetHandle, Asset};
use crate::util::containers::HandleMap;

#[derive(Clone, Debug)]
pub(crate) struct Config {
    value: u32,
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

    fn load_to_cache(&mut self, _path: String) -> AssetHandle {
        //TODO: load config from file
        let config = Config { value: 0 };
        self.cache.insert(config)
    }

    fn load_direct(_path: String) -> Config {
        //TODO: load config from file
        let config = Config { value: 0 };
        config
    }

    fn load_from_cache(&self, handle: AssetHandle) -> Config {
        //TODO: validate bounds
        self.cache.get(handle)
    }

    fn purge_from_cache(&mut self, handle: AssetHandle) {
        self.cache.erase(handle);
    }
}
