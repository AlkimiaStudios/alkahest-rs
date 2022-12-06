use super::{AssetManager, AssetHandle, Asset};
use crate::util::containers::{ContainerError, HandleMap};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use string_interner::{StringInterner, DefaultSymbol};
use image::io::Reader;
use image::EncodableLayout;
use std::io::Cursor;

#[derive(Clone, Debug)]
pub struct TextureData {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Asset for TextureData{}

pub struct TextureManager {
    cache: HandleMap<TextureData>,
    file_map: HashMap<DefaultSymbol, AssetHandle>,
}

impl AssetManager<TextureData> for TextureManager {
    fn init(cache_size: usize) -> TextureManager {
        TextureManager {
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

        let img = image::open(&path).unwrap().into_rgba8();

        let tex_data = TextureData {
            bytes: img.as_bytes().to_vec(),
            width: img.width(),
            height: img.height(),
        };

        let handle = self.cache.insert(tex_data);
        self.file_map.insert(sym, handle);
        return handle;
    }

    fn load_direct(path: String) -> TextureData {
        let img = image::open(&path).unwrap().into_rgba8();

        TextureData {
            bytes: img.as_bytes().to_vec(),
            width: img.width(),
            height: img.height(),
        }
    }

    fn load_from_cache(&self, handle: AssetHandle) -> Result<TextureData, ContainerError> {
        self.cache.get(handle)
    }

    fn purge_from_cache(&mut self, handle: AssetHandle) -> Result<(), ContainerError> {
        self.file_map.retain(|_, v| *v != handle);
        self.cache.erase(handle)
    }
}
