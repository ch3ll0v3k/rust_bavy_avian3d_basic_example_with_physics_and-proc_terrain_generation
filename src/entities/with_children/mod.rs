use avian3d::prelude::*;
use bevy::prelude::*;

// use avian3d::prelude::{AngularVelocity, Collider, RigidBody};
// use avian3d::prelude::{PhysicsSet};

use crate::{
  debug::get_defaul_physic_debug_params,
  AnyObject,
  PhysicsDynamicObject,
  COLLISION_MARGIN,
  DEFAULT_FRICTION,
  DEFAULT_RESTITUTION,
  MASS_UNIT,
};

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MEntityWithChildrenMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MEntityWithChildrenPlugin;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MEntityBigSphere;

impl Plugin for MEntityWithChildrenPlugin {
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
  let pos = Vec3::new(-2.0, 30.0, 2.0);
  let size = Vec3::new(1.0, 1.0, 1.0);

  commands.spawn((
    RigidBody::Dynamic,
    Collider::sphere(1.65),
    CollisionMargin(COLLISION_MARGIN),
    // DebugRender::default().with_collider_color(Color::srgb(1.0, 0.0, 0.0)),
    Transform::from_xyz(pos.x, pos.y, pos.z),
    Mesh3d(meshes.add(Sphere::new(1.65))),
    MeshMaterial3d(materials.add(Color::srgb_u8(127, 255, 0))),
    // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
    LinearVelocity((-Vec3::Y / 10.0) * 5.0),
    Friction::new(DEFAULT_FRICTION),
    Restitution::new(DEFAULT_RESTITUTION),
    Mass(100.0 * MASS_UNIT),
    // ColliderDensity(1000.0),
    PhysicsDynamicObject,
    AnyObject,
    MEntityBigSphere,
    get_defaul_physic_debug_params(),
  ));

  // return;
  commands
    .spawn((
      RigidBody::Dynamic,
      Collider::sphere(0.5),
      CollisionMargin(COLLISION_MARGIN),
      // DebugRender::default().with_collider_color(Color::srgb(1.0, 0.0, 0.0)),
      Transform::from_xyz(pos.x, pos.y, pos.z),
      Mesh3d(meshes.add(Sphere::new(0.5))),
      MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
      AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
      LinearVelocity((-Vec3::Y / 10.0) * 5.0),
      Friction::new(DEFAULT_FRICTION),
      Restitution::new(DEFAULT_RESTITUTION),
      Mass(10.0 * MASS_UNIT),
      // ColliderDensity(1000.0),
      PhysicsDynamicObject,
      AnyObject,
      get_defaul_physic_debug_params(),
    ))
    .with_children(|children| {
      children.spawn((
        Collider::sphere(0.25),
        CollisionMargin(COLLISION_MARGIN),
        // DebugRender::default().with_collider_color(Color::srgb(0.0, 1.0, 0.0)),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 0, 255))),
        Mesh3d(meshes.add(Sphere::new(0.25))),
        Transform::from_xyz(1.0, 0.0, 0.0),
        Mass(1.0 / 10.0),
        PhysicsDynamicObject,
        AnyObject,
        get_defaul_physic_debug_params(),
      ));
      children.spawn((
        Collider::sphere(0.25),
        CollisionMargin(COLLISION_MARGIN),
        // DebugRender::default().with_collider_color(Color::srgb(0.0, 0.0, 1.0)),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 255))),
        Mesh3d(meshes.add(Sphere::new(0.25))),
        Transform::from_xyz(-1.0, 0.0, 0.0),
        Mass(0.5 / 10.0),
        PhysicsDynamicObject,
        AnyObject,
        get_defaul_physic_debug_params(),
      ));
    });
}

// prettier-ignore
fn update(
  // mut terrain: Query<&mut Transform, With<MEntityWithChildrenMarker>>
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
