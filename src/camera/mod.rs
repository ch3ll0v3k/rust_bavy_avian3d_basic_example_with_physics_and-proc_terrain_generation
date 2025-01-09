// prettier-ignore

use bevy::{
  prelude::*,
  color::palettes::css::RED,
  // core_pipeline::{bloom::Bloom, tonemapping::Tonemapping, Skybox},
  math::vec3,
  pbr::{FogVolume, VolumetricFog, VolumetricLight},
};

use bevy::{
  app::FixedUpdate,
  math::{ Vec3, VectorSpace },
  prelude::{ PerspectiveProjection, Projection, TransformPoint, Without },
};

// prettier-ignore
use bevy::{
  prelude::{ in_state, Camera, Camera3d, ClearColor, ClearColorConfig, Component, Deref, With },
};

// prettier-ignore
use bevy::{ 
  prelude::{ DerefMut, IntoSystemConfigs, KeyCode, Query, Res, Resource, Transform }, 
};

// prettier-ignore
use bevy::{ 
  input::common_conditions::input_just_pressed, 
  utils::default
};

// prettier-ignore
use bevy::{
  app::{ App, Plugin, Startup, Update },
  core::Name,
  core_pipeline::prepass::{ DepthPrepass, NormalPrepass },
};

// prettier-ignore
use bevy_render::camera::{PhysicalCameraParameters, RenderTarget};

use crate::{
  m_lib::colors,
  materials,
  player::PlayerMarker,
  post_processing_pipiline::test_example::{
    // CustomPostProcessSettings,
    // TestExamplePostProcessPlugin,
  },
  state::MGameState,
};

#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

#[derive(Component, Debug, PartialEq, Eq)]
pub struct PlayerCameraMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct ViewCameraMarker;

pub struct CameraPlugin;

const USE_PIPELINE: bool = false;

// prettier-ignore
impl Plugin for CameraPlugin {

  fn build(&self, app: &mut App) {

    app
      .insert_resource(Parameters(PhysicalCameraParameters {
          aperture_f_stops: 1.0,
          shutter_speed_s: 1.0 / 125.0,
          sensitivity_iso: 100.0,
          sensor_height: 0.01866,
          // main:
          //   aperture_f_stops: 1.0,
          //   shutter_speed_s: 1.0 / 125.0,
          //   sensitivity_iso: 100.0,
          //   sensor_height: 0.01866,
      }));

    // if( USE_PIPELINE ){
    //   app
    //   .add_plugins(TestExamplePostProcessPlugin);
    // }

    app
      .add_systems(Startup, startup)
      .add_systems(FixedUpdate, update)
      .add_systems(Update,
        (
          switch_camera,
        )
          // .run_if(in_state(MGameState::Running))
          .run_if(input_just_pressed(KeyCode::KeyC))
      );

  }
}

fn switch_camera(mut query: Query<&mut Camera>) {
  for mut camera in query.iter_mut() {
    camera.is_active = !camera.is_active;
  }
}

// prettier-ignore
pub fn get_player_camera() -> (
  Name, 
  Camera3d, 
  Camera,
  DistanceFog,
  PerspectiveProjection,
  Transform, 
  // Tonemapping,
  // Bloom,  
  PlayerCameraMarker, 
  // DepthPrepass, 
  // NormalPrepass,
  // CustomPostProcessSettings
) {
  (
    Name::new("p_player_camera_t"),
    Camera3d{
      ..default()
    },
    Camera{
      is_active: true,
      clear_color: ClearColorConfig::default(),
      hdr: true,
      order: 1,
      ..default()
    },
    DistanceFog {
      color: Color::srgba(0.75, 0.75, 0.75, 0.75),
      // falloff: FogFalloff::ExponentialSquared { density: 0.0002 },
      // falloff: FogFalloff::Exponential { density: 0.00001 },
      falloff: FogFalloff::Linear {
        start: 2_000.0,
        end: 10_000.0,
      },
      ..default()
    },
    PerspectiveProjection {
      near: 0.001,
      ..default()
    },
    Transform::from_xyz(0.0, 100.0, 2.0), // .looking_at(POS, Vec3::Y),
    // Tonemapping::TonyMcMapface,
    // Bloom::default(),
    PlayerCameraMarker,
    // DepthPrepass,
    // NormalPrepass,
    // CustomPostProcessSettings {
    //   cam_y: 0.1,
    //   ..default()
    // },
  )
}

// prettier-ignore
pub fn get_view_camera() -> (
  Name, 
  Camera3d, 
  Camera,
  PerspectiveProjection,
  Transform, 
  ViewCameraMarker, 
  // DepthPrepass, 
  // NormalPrepass,
  // CustomPostProcessSettings
) {
  (
    Name::new("p_view_camera_t"),
    Camera3d{
      ..default()
    },
    Camera{
      
      is_active: false,
      clear_color: ClearColorConfig::default(),
      order: 0,
      ..default()
    },
    PerspectiveProjection {
      near: 0.001,
      ..default()
    },
    Transform::from_xyz(50.0, 100.0, 45.0).looking_at(Vec3::ZERO, Vec3::Y),
    ViewCameraMarker,
    // DepthPrepass,
    // NormalPrepass,
  )
}

// prettier-ignore
fn startup() {
  
}

// prettier-ignore
fn update(
  // mut query_view_camera: Query<&mut Transform, (With<ViewCameraMarker>, Without<PlayerMarker>, )>,
  // query_player: Query<&Transform, (With<PlayerMarker>, Without<ViewCameraMarker>, )>
) {

  // let mut view_trans = query_view_camera.single_mut();
  // let p_trans = query_player.single();

  // // dbgln!("{}", p_trans.translation);

  // view_trans.translation.x = p_trans.translation.x + 24.0;
  // view_trans.translation.y = p_trans.translation.y + 14.0;
  // view_trans.translation.z = p_trans.translation.z + 24.0;

  // view_trans.look_at(p_trans.translation, Vec3::Y);
  // // view_trans.looking_at(p_trans.translation, Vec3::Y);

}

// fn debug_render_targets(q: Query<&PlayerMarker>) {
//   for camera in &q {
//     match &camera.target {
//       RenderTarget::Window(wid) => {
//         eprintln!("Camera renders to window with id: {:?}", wid);
//       }
//       RenderTarget::Image(handle) => {
//         eprintln!("Camera renders to image asset with id: {:?}", handle);
//       }
//       RenderTarget::TextureView(_) => {
//         eprintln!("This is a special camera that outputs to something outside of Bevy.");
//       }
//     }
//   }
// }
