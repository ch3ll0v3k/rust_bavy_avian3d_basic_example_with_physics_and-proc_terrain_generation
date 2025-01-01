// use avian3d::prelude::*;
use bevy::prelude::*;

use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct AudioCachePlugin {
  pub audio_cache: HashMap<String, Handle<AudioSource>>,
}

impl AudioCachePlugin {
  pub fn new() -> Self {
    Self {
      audio_cache: HashMap::new(),
    }
  }
}
