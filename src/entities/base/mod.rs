use avian3d::prelude::*;
use bevy::prelude::*;

// use avian3d::prelude::{AngularVelocity, Collider, RigidBody};
// use avian3d::prelude::{PhysicsSet};

use crate::{
  debug::get_defaul_physic_debug_params,
  AnyObject,
  PhysicsDynamicObject,
  COLLISION_MARGIN,
};

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MEntityBaseMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MEntityBasePlugin;

impl Plugin for MEntityBasePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, startup);
    app.add_systems(Update, update);
  }
}

pub type TPhyBundle = (
  RigidBody,
  Collider,
  CollisionMargin,
  // Mesh3d,
  // MeshMaterial3d<StandardMaterial>,
  Transform,
  AngularVelocity,
  LinearVelocity,
  Mass,
  PhysicsDynamicObject,
  AnyObject,
  DebugRender,
);

fn create_dynamic_cuboid(
  // meshes: &mut ResMut<Assets<Mesh>>,
  // materials: &mut ResMut<Assets<StandardMaterial>>,
  pos: Vec3,
  size: Vec3
) -> TPhyBundle {
  // prettier-ignore
  let phy_t: TPhyBundle = (
    RigidBody::Dynamic,
    Collider::cuboid(size.x, size.y, size.z),
    CollisionMargin(COLLISION_MARGIN),
    Transform::from_xyz(pos.x, pos.y, pos.z),
    AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
    LinearVelocity(Vec3::X / 10.0 * -1.0),
    Mass(2.0),
    PhysicsDynamicObject,
    AnyObject,
    get_defaul_physic_debug_params(),
  );

  phy_t
}

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {
  {
    return;
    let pos = Vec3::new(-2.0, 20.0, 2.0);
    let size = Vec3::new(1.0, 1.0, 1.0);

    // prettier-ignore
    let u_id_0 = commands.spawn((
      create_dynamic_cuboid( pos, size ),
      Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
      MeshMaterial3d(materials.add(Color::srgb_u8(25, 100, 25))),
    )).id();
  }

  {
    let pos = Vec3::new(2.0, 25.0, -2.0);
    let size = Vec3::new(1.0, 1.0, 1.0);

    // prettier-ignore
    let u_id_1 = commands.spawn((
      create_dynamic_cuboid( pos, size ),
      Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
      MeshMaterial3d(materials.add(Color::srgb_u8(100, 25, 25))),
    )).id();
  }

  // let i_id_1 = commands
  //   .spawn((
  //     RigidBody::Dynamic,
  //     Collider::cuboid(1.0, 1.0, 1.0),
  //     CollisionMargin(COLLISION_MARGIN),
  //     MeshMaterial3d(materials.add(Color::srgb_u8(25, 50, 25))),
  //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
  //     Transform::from_xyz(0.0, 7.0, 0.0),
  //     // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
  //     // LinearVelocity(Vec3::X / 10.0),
  //     PhysicsDynamicObject,
  //     AnyObject,
  //     get_defaul_physic_debug_params(),
  //   ))
  //   .id();

  // Connect the bodies with a fixed joint
  // commands.spawn(FixedJoint::new(u_id_0, i_id_1));
  // let v = commands.spawn(DistanceJoint::new(u_id_0, i_id_1));
  // commands.spawn(RevoluteJoint::new(u_id_0, i_id_1));
  // v.with_com
}

// prettier-ignore
fn update(
  // mut terrain: Query<&mut Transform, With<MEntityBaseMarker>>
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
