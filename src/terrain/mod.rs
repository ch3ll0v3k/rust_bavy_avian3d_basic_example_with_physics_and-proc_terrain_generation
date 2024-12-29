use avian3d::prelude::*;
use bevy::prelude::*;

// use avian3d::prelude::{AngularVelocity, Collider, RigidBody};
// use avian3d::prelude::{PhysicsSet};

use crate::{ debug::get_defaul_physic_debug_params, AnyObject, PhysicsStaticObject };

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MTerrainMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MTerrainPlugin;

impl Plugin for MTerrainPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, startup);
    app.add_systems(Update, update);
  }
}

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {
  // commands.spawn((
  //   RigidBody::Static,
  //   Collider::cylinder(20.0, 0.1),
  //   CollisionMargin(0.05),
  //   get_defaul_physic_debug_params(),
  //   Mesh3d(meshes.add(Cylinder::new(20.0, 0.1))),
  //   MeshMaterial3d(materials.add(Color::WHITE)),
  //   Transform::from_xyz(0.0, -2.0, 0.0),
  //   PhysicsStaticObject,
  //   AnyObject,
  // ));
}

// prettier-ignore
fn update(
  // mut terrain: Query<&mut Transform, With<MTerrainMarker>>
) {
  // let mut transform = terrain.single_mut();
  // transform.rotate_local_y(0.01);

  // match transform {
  //   Transform::Perspective(persp) => {
  //     // we have a perspective projection
  //   }
  //   Transform::Orthographic(ortho) => {
  //     // we have an orthographic projection
  //   }
  // }
}
