use std::fs;

// prettier-ignore
use bevy::{
  prelude::{ Resource, Component, ResMut, Res, Commands },
  app::{ App, Plugin, Startup, Update, PluginGroup },
  asset::{ AssetServer, Assets, Handle },
  audio::{ AudioPlayer, AudioPlugin, AudioSource, PlaybackMode, PlaybackSettings, Volume },
  utils::{ default }
};

use crate::{ asset_loader::audio_cache::{ cache_load_audio, AudioCache }, sys_paths };

use sys_paths::audio::EAudio;

#[derive(Resource)]
struct SoundtrackPlayer {
  track_list: Vec<Handle<AudioSource>>,
}

#[derive(Component)]
struct FadeIn;

impl SoundtrackPlayer {
  fn new(track_list: Vec<Handle<AudioSource>>) -> Self {
    Self { track_list }
  }
}

pub struct AmbientAudioPlugin;

// prettier-ignore
impl Plugin for AmbientAudioPlugin {

  fn build(&self, app: &mut App) {

    // app
    //   .insert_resource(debug::config())
    //   .insert_resource(window::config());

    app
      .add_systems(Startup, startup)
      .add_systems(Update, update);

  }
}

// prettier-ignore
fn startup(
  mut res_mut_audio_cache: Option<ResMut</*res_mut_texture_cache::*/AudioCache>>,
  asset_server: Res<AssetServer>,
  mut commands: Commands,
) {
  let audio_hashmap: &mut ResMut<AudioCache> = res_mut_audio_cache.as_mut().unwrap();
  // let track_1: Handle<AudioSource> = asset_server.load::<AudioSource>(sys_paths::sounds::EPaths::EnvOne.as_str());
  let track_1 = cache_load_audio(
    audio_hashmap, 
    &asset_server, 
    EAudio::EnvOne.as_str(),
    false
  );

  // all options are same as default

  commands.spawn((
    AudioPlayer(track_1),
    PlaybackSettings {
      mode: PlaybackMode::Loop,
      volume: Volume::default(),
      ..default()
    },
    // FadeIn,
  ));

  // commands.spawn(AudioPlayer::new(track_1 ));
  // let audio  = AudioPlayer::new(track_1);
  // commands.spawn(audio);

}

// prettier-ignore
fn update() {}
