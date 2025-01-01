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

#[derive(Resource, Default)]
pub struct TextureCache {
  pub texture_cache: HashMap<String, Handle<Image>>,
}

impl TextureCache {
  pub fn new() -> Self {
    Self {
      texture_cache: HashMap::new(),
    }
  }
}

// prettier-ignore
fn _load_texture_with_common_settings(
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
pub fn cache_load_texture(
  // mut texture_cache: Option<ResMut</*texture_cache::*/TextureCache>>,
  audio_hashmap: &mut ResMut<TextureCache>,
  asset_server: &Res<AssetServer>,
  path: &str,
  with_settings: bool
) -> Handle<Image> {
   
  let mut texture_handle: Handle<Image>;
  let cache_path: String = path.to_string();
    // println!("capacity: {:?}", audio_hashmap.hash_map.capacity());
    
  if let Some(handle_image) = audio_hashmap.texture_cache.get(&cache_path) {
    println!("audio_hashmap.texture_cache.get(&({cache_path}) => found ...");
    return handle_image.clone();      
  }
  
  println!("audio_hashmap.texture_cache.get(&({cache_path}) => not found: update cache ...");
  texture_handle = _load_texture_with_common_settings(asset_server, path, with_settings);
  audio_hashmap.texture_cache.insert(cache_path, texture_handle.clone());
 
  texture_handle

  // let mut texture_handle: Handle<Image>;
  // let cache_path: String = path.to_string();
  // if let Some(audio_hashmap) = &mut texture_cache {
  //   // println!("capacity: {:?}", audio_hashmap.hash_map.capacity());
    
  //   if let Some(handle_image) = audio_hashmap.texture_cache.get(&cache_path) {
  //     println!("audio_hashmap.texture_cache.get(&({cache_path}) => found ...");
  //     return handle_image.clone();      
  //   }
    
  //   println!("audio_hashmap.texture_cache.get(&({cache_path}) => not found: update cache ...");
  //   texture_handle = _load_texture_with_common_settings(asset_server, path, with_settings);
  //   audio_hashmap.texture_cache.insert(cache_path, texture_handle.clone());
    
  // }else{
  //   println!("audio_hashmap.texture_cache.get(&({cache_path}) => hash-map resource is not available: fallback to regular asset_loader::<T>() ...");
  //   texture_handle = _load_texture_with_common_settings(asset_server, path, with_settings);
  // }

  // texture_handle
  
}
