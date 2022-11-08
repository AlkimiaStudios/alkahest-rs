use super::{AssetManager, AssetHandle, Asset};
use crate::util::containers::{ContainerError, HandleMap};
use std::fs;
use std::collections::HashMap;
use string_interner::{StringInterner, DefaultSymbol};

#[derive(Clone, Debug)]
pub struct Shader {
    pub id: u32,
    pub code: String,
}

impl Asset for Shader{}

pub struct ShaderManager {
    cache: HandleMap<Shader>,
    file_map: HashMap<DefaultSymbol, AssetHandle>,    
}

impl AssetManager<Shader> for ShaderManager {
    fn init(cache_size: usize) -> ShaderManager {
        ShaderManager {
            cache: HandleMap::new(0, cache_size),
            file_map: HashMap::new(),
        }
    }

    fn load_to_cache(&mut self, path: String) -> AssetHandle {
        let mut interner = StringInterner::default();
        let sym = interner.get_or_intern(&path);
        if self.file_map.contains_key(&sym) {
            if let Some(handle) = self.file_map.get(&sym) {
                return handle.clone();
            }
        }

        let code = fs::read_to_string(&path).unwrap_or(String::from(""));
        let handle = self.cache.insert(Shader { id: 0, code });
        self.file_map.insert(sym, handle);
        return handle;
    }

    fn load_direct(path: String) -> Shader {
        let code = fs::read_to_string(path).unwrap_or(String::from(""));
        Shader { id: 0, code }
    }

    fn load_from_cache(&self, handle: AssetHandle) -> Result<Shader, ContainerError> {
        self.cache.get(handle)
    }

    fn purge_from_cache(&mut self, handle: AssetHandle) -> Result<(), ContainerError> {
        self.file_map.retain(|_, v| *v != handle);
        self.cache.erase(handle)
    }
}
