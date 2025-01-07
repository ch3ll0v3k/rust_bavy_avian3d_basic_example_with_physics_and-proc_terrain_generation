// prettier-ignore
use bevy::{ 
  app::{ App, Plugin, Startup, Update }, core::Name, core_pipeline::prepass::{DepthPrepass, NormalPrepass}, prelude::{ Camera3d, Component, Deref, DerefMut, Resource, Transform }, utils::default
};

// prettier-ignore
use bevy_render::{
  camera::PhysicalCameraParameters,
};

use crate::{
  materials,
  post_processing_pipiline::test_example::{
    CustomPostProcessSettings,
    TestExamplePostProcessPlugin,
  },
};

#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

#[derive(Component, Debug, PartialEq, Eq)]
pub struct CameraMarker;

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

    if( USE_PIPELINE ){
      app
      .add_plugins(TestExamplePostProcessPlugin);
    }

    app
      .add_systems(Startup, startup)
      .add_systems(Update, update);
  }
}

// prettier-ignore
pub fn get_camera() -> (
  Name, 
  Camera3d, 
  Transform, 
  CameraMarker, 
  DepthPrepass, 
  NormalPrepass,
  CustomPostProcessSettings
) {
  (
    Name::new("p_player_camera_t"),
    Camera3d::default(),
    // Transform::from_xyz(0.0, 6.0, 0.0), // .looking_at(POS, Vec3::Y),
    Transform::from_xyz(0.0, 1.0, 0.0), // .looking_at(POS, Vec3::Y),
    CameraMarker,
    DepthPrepass,
    NormalPrepass,
    CustomPostProcessSettings {
      // intensity: 0.05,
      // set_r: 0.1,
      // set_g: 0.2,
      // set_b: 0.3,
      cam_y: 0.1,
      ..default()
    },
  )
}

// prettier-ignore
fn startup() {
  
}

// prettier-ignore
fn update() {

}
