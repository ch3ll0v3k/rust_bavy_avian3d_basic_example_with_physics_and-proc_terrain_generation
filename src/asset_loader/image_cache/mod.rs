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
  utils::default,
};

use std::collections::HashMap;

use crate::dbgln;

#[derive(Resource, Default)]
pub struct ImageCache {
  pub image_cache: HashMap<String, Handle<Image>>,
}

impl ImageCache {
  pub fn new() -> Self {
    Self {
      image_cache: HashMap::new(),
    }
  }
}

// prettier-ignore
fn _load_image_with_common_settings(
  asset_server: &Res<AssetServer>,
  path: &str,
  with_settings: bool
) -> Handle<Image> {

  if !with_settings {
    return asset_server.load(path);
  }

  return asset_server.load_with_settings(path, |settings: &mut _| {
    *settings = ImageLoaderSettings {
      sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        // address_mode_w: ImageAddressMode::ClampToBorder,
        mag_filter: ImageFilterMode::Linear,
        ..default()
      }),
      ..default()
    };
  });
}

// prettier-ignore
pub fn cache_load_image(
  // mut image_cache: Option<ResMut</*image_cache::*/ImageCache>>,
  image_hashmap: &mut ResMut<ImageCache>,
  asset_server: &Res<AssetServer>,
  path: &str,
  with_settings: bool
) -> Handle<Image> {
   
  let mut image_handle: Handle<Image>;
  let cache_path: String = path.to_string();
    // dbgln!("capacity: {:?}", image_hashmap.hash_map.capacity());
    
  if let Some(handle_image) = image_hashmap.image_cache.get(&cache_path) {
    // dbgln!("image_hashmap.image_cache.get(&({cache_path}) => found ...");
    return handle_image.clone();      
  }
  
  // dbgln!("image_hashmap.image_cache.get(&({cache_path}) => not found: update cache ...");
  image_handle = _load_image_with_common_settings(asset_server, path, with_settings);
  image_hashmap.image_cache.insert(cache_path, image_handle.clone());
 
  image_handle
  
}
