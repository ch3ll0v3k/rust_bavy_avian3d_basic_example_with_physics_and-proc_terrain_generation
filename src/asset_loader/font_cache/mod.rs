// use avian3d::prelude::*;
// use bevy::prelude::*;

use bevy::{
  app::{ App, Plugin },
  asset::{ AssetServer, Handle },
  image::{
    Image,
    ImageAddressMode,
    ImageFilterMode,
    ImageLoaderSettings,
    ImageSampler,
    ImageSamplerDescriptor,
  },
  prelude::{ Component, Res, ResMut, Resource },
  text::Font,
  utils::default,
};

use std::collections::HashMap;

use crate::dbgln;

#[derive(Resource, Default)]
pub struct FontCache {
  pub font_cache: HashMap<String, Handle<Font>>,
}

impl FontCache {
  pub fn new() -> Self {
    Self {
      font_cache: HashMap::new(),
    }
  }
}

// prettier-ignore
fn _load_font_with_common_settings(
  asset_server: &Res<AssetServer>,
  path: &str,
  with_settings: bool
) -> Handle<Font> {
  
  if !with_settings {
    let handle_font_source: Handle<Font> = asset_server.load::<Font>(path);
    return handle_font_source;
  }
  let handle_font_source: Handle<Font> = asset_server.load::<Font>(path);
  return handle_font_source;

}

// prettier-ignore
pub fn cache_load_font(
  // mut font_cache: Option<ResMut</*font_cache::*/FontCache>>,
  font_hashmap: &mut ResMut<FontCache>,
  asset_server: &Res<AssetServer>,
  path: &str,
  with_settings: bool
) -> Handle<Font> {
   
  let mut font_handle: Handle<Font>;
  let cache_path: String = path.to_string();
    // dbgln!("capacity: {:?}", font_hashmap.hash_map.capacity());
    
  if let Some(handle_font) = font_hashmap.font_cache.get(&cache_path) {
    // dbgln!("font_hashmap.font_cache.get(&({cache_path}) => found ...");
    return handle_font.clone();      
  }
  
  // dbgln!("font_hashmap.font_cache.get(&({cache_path}) => not found: update cache ...");
  font_handle = _load_font_with_common_settings(asset_server, path, with_settings);
  font_hashmap.font_cache.insert(cache_path, font_handle.clone());
 
  font_handle
  
}
