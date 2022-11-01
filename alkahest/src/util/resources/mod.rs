mod config_manager;
pub use config_manager::*;

use crate::util::containers::Handle;

use Handle as AssetHandle;

pub(crate) trait AssetManager<T> where T: Asset + Clone {
    fn init(cache_size: usize) -> Self;
    fn load_to_cache(&mut self, path: String) -> AssetHandle;
    fn load_direct(path: String) -> T;
    fn load_from_cache(&self, handle: AssetHandle) -> T;
    fn purge_from_cache(&mut self, handle: AssetHandle);
}

pub(crate) trait Asset {}
