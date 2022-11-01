mod config_manager;
pub(crate) use config_manager::*;

#[derive(Copy, Clone)]
pub(crate) struct __AssetHandle {
    index: u32,
    asset_type: u32,
}

#[derive(Copy, Clone)]
pub(crate) union AssetHandle {
    _fields: __AssetHandle,
    value: u64,
}

pub(crate) trait AssetManager<T> where T: Asset + Clone {
    fn load_to_cache(&mut self, path: String) -> AssetHandle;
    fn load_direct(path: String) -> T;
    fn load_from_cache(&self, handle: AssetHandle) -> T;
    fn purge_from_cache(&mut self, handle: AssetHandle);
}

pub(crate) trait Asset {}
