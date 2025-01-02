// use avian3d::prelude::*;
// use bevy::prelude::*;

use bevy::audio::AudioPlugin;

use bevy::{
  app::{ App, Plugin },
  asset::{ AssetServer, Handle },
  audio::{ AudioPlayer, AudioSource, PlaybackSettings, PlaybackMode, Volume },
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
pub struct AudioCache {
  pub audio_cache: HashMap<String, Handle<AudioSource>>,
}

impl AudioCache {
  pub fn new() -> Self {
    Self {
      audio_cache: HashMap::new(),
    }
  }
}

// prettier-ignore
fn _load_audio_with_common_settings(
  asset_server: &Res<AssetServer>,
  path: &str,
  with_settings: bool
) -> Handle<AudioSource> {
  
  if !with_settings {
    let handle_audio_source: Handle<AudioSource> = asset_server.load::<AudioSource>(path);
    return handle_audio_source;
  }
  let handle_audio_source: Handle<AudioSource> = asset_server.load::<AudioSource>(path);
  return handle_audio_source;

}

// prettier-ignore
pub fn cache_load_audio(
  // mut audio_cache: Option<ResMut</*audio_cache::*/AudioCache>>,
  audio_hashmap: &mut ResMut<AudioCache>,
  asset_server: &Res<AssetServer>,
  path: &str,
  with_settings: bool
) -> Handle<AudioSource> {
   
  let mut audio_handle: Handle<AudioSource>;
  let cache_path: String = path.to_string();
    // dbgln!("capacity: {:?}", audio_hashmap.hash_map.capacity());
    
  if let Some(handle_audio) = audio_hashmap.audio_cache.get(&cache_path) {
    dbgln!("audio_hashmap.audio_cache.get(&({cache_path}) => found ...");
    return handle_audio.clone();      
  }
  
  dbgln!("audio_hashmap.audio_cache.get(&({cache_path}) => not found: update cache ...");
  audio_handle = _load_audio_with_common_settings(asset_server, path, with_settings);
  audio_hashmap.audio_cache.insert(cache_path, audio_handle.clone());
 
  audio_handle

  // let mut audio_handle: Handle<Image>;
  // let cache_path: String = path.to_string();
  // if let Some(audio_hashmap) = &mut audio_cache {
  //   // dbgln!("capacity: {:?}", audio_hashmap.hash_map.capacity());
    
  //   if let Some(handle_audio) = audio_hashmap.audio_cache.get(&cache_path) {
  //     dbgln!("audio_hashmap.audio_cache.get(&({cache_path}) => found ...");
  //     return handle_audio.clone();      
  //   }
    
  //   dbgln!("audio_hashmap.audio_cache.get(&({cache_path}) => not found: update cache ...");
  //   audio_handle = _load_audio_with_common_settings(asset_server, path, with_settings);
  //   audio_hashmap.audio_cache.insert(cache_path, audio_handle.clone());
    
  // }else{
  //   dbgln!("audio_hashmap.audio_cache.get(&({cache_path}) => hash-map resource is not available: fallback to regular asset_loader::<T>() ...");
  //   audio_handle = _load_audio_with_common_settings(asset_server, path, with_settings);
  // }

  // audio_handle
  
}
