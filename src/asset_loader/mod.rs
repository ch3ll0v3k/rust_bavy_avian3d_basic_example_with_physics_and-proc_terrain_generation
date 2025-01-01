// prettier-ignore
use bevy::{
  app::{ App, Plugin },
  prelude::{ Component },
};

use crate::{ sys_paths };

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MAssetLoaderPlugin;

pub mod image_cache;
pub mod audio_cache;
pub mod font_cache;

// prettier-ignore
impl Plugin for MAssetLoaderPlugin {

  fn build(&self, app: &mut App) {
    // app
    // .add_systems(Startup, startup)
    // .add_systems(Update, update);

    app
      .insert_resource(image_cache::ImageCache::new())
      .insert_resource(audio_cache::AudioCache::new())
      .insert_resource(font_cache::FontCache::new());

  }
}
