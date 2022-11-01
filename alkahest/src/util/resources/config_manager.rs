use super::{AssetManager, AssetHandle, Asset};

pub(crate) struct ConfigManager;

#[derive(Clone)]
pub(crate) struct Config {
    value: u32,
}

impl Asset for Config {}

impl AssetManager<Config> for ConfigManager {
    fn load_to_cache(&mut self, _path: String) -> AssetHandle {
        let handle = AssetHandle { value: 0 };
        handle
    }

    fn load_direct(_path: String) -> Config {
        let config = Config { value: 0 };
        config
    }

    fn load_from_cache(&self, _handle: AssetHandle) -> Config {
        let config = Config { value: 0 };
        config
    }

    fn purge_from_cache(&mut self, _handle: AssetHandle) {

    }
}
