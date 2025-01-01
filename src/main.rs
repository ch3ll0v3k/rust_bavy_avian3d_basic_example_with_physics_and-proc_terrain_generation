#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_braces)]
#![allow(unused_parens)]

use asset_loader::audio_cache::cache_load_audio;
use asset_loader::audio_cache::AudioCache;
// use avian3d::debug_render::DebugRender;
use avian3d::debug_render::PhysicsDebugPlugin;
use avian3d::prelude::*;
use avian3d::PhysicsPlugins;
// use bevy::audio::AudioPlugin;
use bevy::audio::AudioPlayer;
use bevy::color::palettes::css::BLACK;
use bevy::color::palettes::css::SILVER;
use bevy::color::palettes::css::WHITE_SMOKE;
use bevy::image::ImageAddressMode;
use bevy::image::ImageFilterMode;
use bevy::image::ImageLoaderSettings;
use bevy::image::ImageSampler;
use bevy::image::ImageSamplerDescriptor;
use bevy::time::common_conditions::on_timer;
use camera::CameraParentMarker;
use noise::{ BasicMulti, NoiseFn, Perlin };
// use bevy_window::WindowLevel;

use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy::window::PresentMode::*;
use bevy::window::WindowMode::*;
use bevy_window::WindowResolution;
use bevy::color::palettes::tailwind::*;
// use bevy::{ color::palettes::tailwind::* };

use bevy::math::Affine2;

use camera::CameraMarker;
use debug::get_defaul_physic_debug_params;
// use entities::with_children::MEntityBigSphere;
use lights::MPointLightMarker;

use std::collections::HashMap;
use std::time::Duration;

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

use markers::m_avian::*;
use markers::m_bevy::*;
use constants::viewport_settings::*;
use constants::physics_world::*;
use terrain::MTerrainMarker;
use sys_paths::audio::EAudioPaths;
use sys_paths::image::EImagePaths;

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
// This component will be attached to an entity to fade the audio in
#[derive(Component)]
struct FadeIn;

// This component will be attached to an entity to fade the audio in

fn main() {
  App::new()
    // Enable physics
    // .add_plugins((PanOrbitCameraPlugin,))
    // .insert_resource(ClearColor(Color::from(BLUE_200)))
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
          present_mode: AutoNoVsync,
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
    // .add_systems(Update, update)
    // .add_systems(FixedUpdate, update)
    .add_systems(Update, update.run_if(on_timer(Duration::from_millis(1000))))

    .insert_resource(Time::<Fixed>::from_hz(60.0))
    // .add_systems(Startup, play)
    .insert_resource(Gravity(Vec3::NEG_Y * GRAVITY))
    .run();
}

// #[derive(Component, Debug, PartialEq, Eq)]
// pub struct Terrain;

// https://github.com/bevyengine/bevy/blob/v0.15.0/examples/audio/soundtrack.rs

// fn play(
//   mut commands: Commands,
//   soundtrack_player: Res<SoundtrackPlayer>,
//   soundtrack: Query<Entity, With<AudioSink>>
//   // game_state: Res<GameState>
// ) {
//   // commands.spawn((
//   //   AudioPlayer(soundtrack_player.track_list.first().unwrap().clone()),
//   //   // AudioPlayer(track_list.first().unwrap().clone()),
//   //   PlaybackSettings {
//   //     mode: bevy::audio::PlaybackMode::Loop,
//   //     volume: bevy::audio::Volume::default(),
//   //     ..default()
//   //   },
//   //   // FadeIn,
//   // ));
// }

// fn get_base_texture_material(
//   asset_server: &Res<AssetServer>,
//   mut materials: ResMut<Assets<StandardMaterial>>,
//   path: &str
// ) -> (Handle<Image>, StandardMaterial, Handle<StandardMaterial>) {
//   let texture: Handle<Image> = load_base_texture(
//     asset_server,
//     EImagePaths::Base.as_str()
//   );

//   let mut material = StandardMaterial {
//     // base_color: Color::BLACK,
//     base_color_texture: Some(texture.clone()),
//     // https://bevyengine.org/examples/assets/repeated-texture/
//     // uv_transform: Affine2::from_scale(Vec2::new(1.0, 1.0)),
//     // uv_transform: Affine2::from_scale(Vec2::new(2.0, 2.0)),
//     // alpha_mode: AlphaMode::Blend,
//     unlit: false,
//     emissive: LinearRgba::BLACK,
//     // emissive_exposure_weight: 1.0,
//     perceptual_roughness: 0.85,
//     // metallic: 0.0,
//     reflectance: 0.05,
//     // ior: 1.47,
//     ..default()
//   };

//   // material.base_color = Color::srgba_u8(128, 197, 222, 17);

//   // material.base_color_tiling = Vec2::new(2.0, 2.0); // Scale the texture UVs
//   let handle: Handle<StandardMaterial> = materials.add(material.clone());

//   (texture, material, handle)
// }

// fn gen_collinders_if(x: f64, z: f64, dyn_scale: i16) {}

// prettier-ignore
fn setup(
  mut res_mut_audio_cache: Option<ResMut</*res_mut_texture_cache::*/AudioCache>>,
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {

  
  {    
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
        mode: bevy::audio::PlaybackMode::Loop,
        volume: bevy::audio::Volume::default(),
        ..default()
      },
      // FadeIn,
    ));

    // commands.spawn(AudioPlayer::new(track_1 ));

    // let audio  = AudioPlayer::new(track_1);
    // commands.spawn(audio);
  }
  // // let Ok(entity) = query.get_single_mut() else { return; };

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
