mod config_manager;
pub use config_manager::*;
mod tex;
pub use tex::*;
mod shader_manager;
pub use shader_manager::*;

use crate::util::containers::{ContainerError, Handle};

use Handle as AssetHandle;

pub(crate) trait AssetManager<T> where T: Asset + Clone {
    fn init(cache_size: usize) -> Self;
    fn load_to_cache(&mut self, path: String) -> AssetHandle;
    fn load_direct(path: String) -> T;
    fn load_from_cache(&self, handle: AssetHandle) -> Result<T, ContainerError>;
    fn purge_from_cache(&mut self, handle: AssetHandle) -> Result<(), ContainerError>;
}

pub(crate) trait Asset {}
