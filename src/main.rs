#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_braces)]
#![allow(unused_parens)]

#[macro_use]
mod debug_utils;

use avian3d::debug_render::{ PhysicsDebugPlugin, DebugRender };
use avian3d::prelude::*;
use avian3d::PhysicsPlugins;

use bevy::prelude::*;
use asset_loader::audio_cache::{ cache_load_audio, AudioCache };

use bevy::app::{ ScheduleRunnerPlugin, App, Startup, Update };
use bevy::asset::{ AssetServer, Assets, Handle };
use bevy::audio::AudioPlugin;
use bevy::audio::{ AudioPlayer, AudioSource, PlaybackSettings, PlaybackMode, Volume };
use bevy::color::{ Color, palettes::css::*, palettes::tailwind::* };

use bevy::gizmos::AppGizmoBuilder;
use bevy::image::{
  ImageAddressMode,
  ImageFilterMode,
  ImageLoaderSettings,
  ImageSampler,
  ImageSamplerDescriptor,
};

use bevy::math::{ IVec2, Vec2, Vec3 };
use bevy::pbr::StandardMaterial;
use bevy::time::{ Time, Fixed, common_conditions::on_timer };
use bevy::utils::default;
use bevy_window::{
  WindowResolution,
  WindowLevel,
  PresentMode,
  Window,
  WindowPlugin,
  WindowPosition,
};

use bevy::{
  pbr::{ CascadeShadowConfigBuilder, ExtendedMaterial, OpaqueRendererMethod },
  // core_pipeline::{
  //     bloom::BloomSettings,
  //     dof::{ DepthOfFieldMode, DepthOfFieldSettings },
  //     prepass::{ DepthPrepass, NormalPrepass },
  //     tonemapping::Tonemapping,
  //     Skybox,
  // },
};

use instant::Instant;
use noise::{ BasicMulti, NoiseFn, Perlin };

use bevy::ecs::query::QuerySingleError;
use bevy::render::mesh::VertexAttributeValues;
use bevy::window::WindowMode::*;

// use bevy::math::Affine2;
use std::{ collections::HashMap, time::Duration };

mod camera;
mod cubes;
mod debug;
mod lights;
mod markers;
mod constants;
mod terrain;
mod sky;
mod entities;
mod state;
mod sys_paths;
mod asset_loader;
mod m_lib;
mod materials;

use sys_paths::image::EImagePaths;
use camera::CameraMarker;
use debug::get_defaul_physic_debug_params;
use lights::MPointLightMarker;
use markers::{ m_avian::*, m_bevy::* };
use constants::{ viewport_settings::*, physics_world::* };
use terrain::MTerrainMarker;
use sys_paths::audio::EAudioPaths;
use camera::CameraParentMarker;
use m_lib::physics;

const WINDOW_POSITIONS_DEV_SIDE_33_PERCENT: Vec2 = Vec2::new(800.0, 1100.0);
const WINDOW_POSITIONS_DEV_SIDE_50_PERCENT: Vec2 = Vec2::new(950.0, 1100.0);
const USE_WIN_SIZE: Vec2 = WINDOW_POSITIONS_DEV_SIDE_50_PERCENT;

#[derive(Resource)]
struct SoundtrackPlayer {
  track_list: Vec<Handle<AudioSource>>,
}

impl SoundtrackPlayer {
  fn new(track_list: Vec<Handle<AudioSource>>) -> Self {
    Self { track_list }
  }
}

#[derive(Component)]
struct FadeIn;

fn main() {
  dbgln!("App stating...");

  App::new()
    // Enable physics
    // .add_plugins((PanOrbitCameraPlugin,))
    // .insert_resource(ClearColor(Color::from(BLUE_200)))
    // .insert_resource(WindowDescriptor {
    //   present_mode: PresentMode::AutoVsync,
    //   ..default()
    // })
    // .add_plugins(
    //   ScheduleRunnerPlugin::run_loop(
    //     // Run 60 times per second.
    //     Duration::from_secs_f64(1.0 / FARERATE_LIMIT)
    //     // Duration::from_secs_f64(10.0)
    //   )
    // )
    .add_plugins((
      // AssetPlugin::default(),
      // AudioPlugin::default(),
      // LogDiagnosticsPlugin::default(),
      DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          position: WindowPosition::At(IVec2::new(1200, 0)),
          // title: "Bevy Game".to_string(),
          resolution: WindowResolution::new(
            // WP_W / WP_SCALE,
            // WP_H / WP_SCALE
            USE_WIN_SIZE.x,
            USE_WIN_SIZE.y
          ).with_scale_factor_override(1.0),
          present_mode: PresentMode::AutoNoVsync,
          // present_mode: PresentMode::AutoVsync,
          // present_mode: PresentMode::Immediate,
          // mode: Fullscreen(MonitorSelection::Primary),
          // mode: BorderlessFullscreen(MonitorSelection::Primary),
          // resizable: false,
          // fit_canvas_to_parent: true,
          // fullsize_content_view: true,
          ..Default::default()
        }),
        ..Default::default()
      }), // .set(WindowPlugin {}),
      PhysicsPlugins::default(),
      PhysicsDebugPlugin::default(),
      asset_loader::MAssetLoaderPlugin,
      cubes::CubesPlugin,
      debug::DebugPlugin,
      camera::CameraPlugin,
      lights::MLightsPlugin,
      terrain::MTerrainPlugin,
      sky::MSkyPlugin,
      entities::base::MEntityBasePlugin,
      entities::with_children::MEntityWithChildrenPlugin,
      state::MGameStatePlugin,
    ))
    .insert_gizmo_config(
      PhysicsGizmos {
        aabb_color: Some(Color::WHITE),
        ..default()
      },
      GizmoConfig::default()
    )
    .add_systems(Startup, setup)
    .add_systems(Update, update.run_if(on_timer(Duration::from_millis(1000))))
    .insert_resource(Gravity(physics::get_gravity_vec3()))
    .run();
}

// prettier-ignore
fn setup(
  mut res_mut_audio_cache: Option<ResMut</*res_mut_texture_cache::*/AudioCache>>,
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {

  let audio_hashmap: &mut ResMut<AudioCache> = res_mut_audio_cache.as_mut().unwrap();
  // let track_1: Handle<AudioSource> = asset_server.load::<AudioSource>(sys_paths::sounds::EPaths::EnvOne.as_str());
  let track_1 = cache_load_audio(
    audio_hashmap, 
    &asset_server, 
    EAudioPaths::EnvOne.as_str(),
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
fn update(
  // mut inner_mapper_mut: Option<ResMut<InnerMapper>>,
  // // inner_mapper_read: Res<InnerMapper>,
  // // inner_mapper: Res<InnerMapper>,
  // // mut q_terrain: Query<&mut Transform, (With<MTerrainMarker>, Without<CameraMarker>)>,
  // q_name: Query<&Name>,
  // mut commands: Commands,
  // mut q_player: Query<&mut Transform, (With<CameraParentMarker>, Without<MTerrainMarker>)>,
  // mut q_terrain: Query<
  //   (Entity, &mut RigidBody, &mut Transform),
  //   (With<MTerrainMarker>, Without<CameraMarker>)
  // >,
) {


  
}
