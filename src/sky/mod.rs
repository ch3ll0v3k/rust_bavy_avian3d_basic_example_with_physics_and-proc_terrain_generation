use bevy::color::palettes::tailwind::*;
use bevy::core_pipeline::Skybox;
use bevy::image::{
  ImageAddressMode,
  ImageFilterMode,
  ImageLoaderSettings,
  ImageSampler,
  ImageSamplerDescriptor,
};
use bevy::pbr::{ NotShadowCaster, NotShadowReceiver };
use bevy::render::mesh::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use wgpu::Face;

// use avian3d::prelude::{AngularVelocity, Collider, RigidBody};
// use avian3d::prelude::{PhysicsSet};

use crate::{ debug::get_defaul_physic_debug_params, AnyObject, PhysicsStaticObject };
use crate::COLLISION_MARGIN;
use crate::sys_paths;

use sys_paths::audio::EAudioPaths;
use sys_paths::image::EImagePaths;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MSkyMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MSkyPlugin;

impl Plugin for MSkyPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, startup);
    app.add_systems(Update, update);
  }
}

fn load_base_texture(asset_server: &Res<AssetServer>, path: &str) -> Handle<Image> {
  let texture_handle: Handle<Image> = asset_server.load_with_settings(path, |s: &mut _| {
    *s = ImageLoaderSettings {
      sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
        // rewriting mode to repeat image,
        // address_mode_u: ImageAddressMode::Repeat,
        // address_mode_v: ImageAddressMode::Repeat,
        // // address_mode_w: ImageAddressMode::ClampToBorder,
        // mag_filter: ImageFilterMode::Linear,
        ..default()
      }),
      ..default()
    };
  });

  texture_handle
}

fn startup(
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {
  // if true {
  //   let cubes: i32 = 3;
  //   for y in 0..cubes {
  //     for x in 0..cubes {
  //       for z in 0..cubes {
  //         let cube = Mesh::from(Cuboid::new(10.0, 10.0, 10.0));
  //         let random_color = Color::srgb(
  //           rand::random::<f32>() * 255.0,
  //           rand::random::<f32>() * 255.0,
  //           rand::random::<f32>() * 255.0
  //         );

  //         commands.spawn((
  //           NotShadowCaster,
  //           NotShadowReceiver,
  //           RigidBody::Dynamic,
  //           Mass(0.1),
  //           CollisionMargin(COLLISION_MARGIN),
  //           Collider::trimesh_from_mesh(&cube).unwrap(),
  //           Mesh3d(meshes.add(cube)),
  //           MeshMaterial3d(materials.add(random_color)),
  //           Transform::from_xyz(
  //             30.0 * (x as f32),
  //             rand::random::<f32>() * 100.0 + 250.0 + (y as f32),
  //             30.0 * (z as f32)
  //           ),
  //           PhysicsStaticObject,
  //           AnyObject,
  //         ));
  //       }
  //     }
  //   }
  // }

  const SIZE_T: f32 = 100_000_000.0;
  const OFFSET_T: f32 = 0.0;

  // {
  //   let sky_seg_down: Handle<Image> = load_base_texture(
  //     &asset_server,
  //     EImagePaths::SkySegDown.as_str()
  //   );
  // }

  {
    let sky_seg_east: Handle<Image> = load_base_texture(
      &asset_server,
      EImagePaths::SkySegEast.as_str()
    );

    let sky_seg_east_mash = Mesh::from(Cuboid::new(SIZE_T, SIZE_T, 1.0));
    // let sky_seg_east_mash = Mesh::from(Plane3d::new(Vec3::Z, Vec2::new(SIZE_T, SIZE_T)));
    commands.spawn((
      NotShadowCaster,
      NotShadowReceiver,
      Mesh3d(meshes.add(sky_seg_east_mash)),
      // MeshMaterial3d(materials.add(Color::WHITE)),
      MeshMaterial3d(
        materials.add(StandardMaterial {
          // base_color: Color::srgb(0.5, 0.8, 1.0), // Sky blue
          // base_color: Color::srgb(255.0, 0.0, 0.0), // Sky blue
          base_color_texture: Some(sky_seg_east.clone()),
          base_color: Color::from(BLUE_200), // Sky blue
          cull_mode: Some(Face::Front),
          // cull_mode: Some(Face::Back),
          unlit: !false,
          double_sided: false,
          ..default()
        })
      ),
      Transform::from_xyz(0.0, OFFSET_T, SIZE_T / 2.0),
      // Transform::from_rotation(Quat::from_rotation_x(3.141592 / 2.0)),
      // Quat::from_rotation_z(2f32),
      PhysicsStaticObject,
      AnyObject,
    ));
  }

  // return;
  {
    let sky_seg_north: Handle<Image> = load_base_texture(
      &asset_server,
      EImagePaths::SkySegNorth.as_str()
    );

    let sky_seg_north_mash = Mesh::from(Cuboid::new(1.0, SIZE_T, SIZE_T));
    commands.spawn((
      NotShadowCaster,
      NotShadowReceiver,
      Mesh3d(meshes.add(sky_seg_north_mash)),
      // MeshMaterial3d(materials.add(Color::WHITE)),
      MeshMaterial3d(
        materials.add(StandardMaterial {
          // base_color: Color::srgb(0.5, 0.8, 1.0), // Sky blue
          // base_color: Color::srgb(255.0, 0.0, 0.0), // Sky blue
          base_color_texture: Some(sky_seg_north.clone()),
          base_color: Color::from(BLUE_200), // Sky blue
          cull_mode: Some(Face::Front), // Render the inside of the sphere
          unlit: !false,
          double_sided: false,
          ..default()
        })
      ),
      Transform::from_xyz(SIZE_T / 2.0, OFFSET_T, 0.0),
      // Transform::from_rotation(Quat::from_rotation_x(3.141592 / 2.0)),
      // Quat::from_rotation_z(2f32),
      PhysicsStaticObject,
      AnyObject,
    ));
  }

  {
    let sky_seg_south: Handle<Image> = load_base_texture(
      &asset_server,
      EImagePaths::SkySegSouth.as_str()
    );
    let sky_seg_south_mesh = Mesh::from(Cuboid::new(1.0, SIZE_T, SIZE_T));
    commands.spawn((
      NotShadowCaster,
      NotShadowReceiver,
      Mesh3d(meshes.add(sky_seg_south_mesh)),
      // MeshMaterial3d(materials.add(Color::WHITE)),
      MeshMaterial3d(
        materials.add(StandardMaterial {
          // base_color: Color::srgb(0.5, 0.8, 1.0), // Sky blue
          // base_color: Color::srgb(255.0, 0.0, 0.0), // Sky blue
          base_color_texture: Some(sky_seg_south.clone()),
          base_color: Color::from(BLUE_200), // Sky blue
          cull_mode: Some(Face::Front), // Render the inside of the sphere
          unlit: !false,
          double_sided: false,
          ..default()
        })
      ),
      Transform::from_xyz(-SIZE_T / 2.0, OFFSET_T, 0.0),
      // Transform::from_rotation(Quat::from_rotation_x(3.141592 / 2.0)),
      // Quat::from_rotation_z(2f32),
      PhysicsStaticObject,
      AnyObject,
    ));
  }

  {
    let sky_seg_west: Handle<Image> = load_base_texture(
      &asset_server,
      EImagePaths::SkySegWest.as_str()
    );

    let sky_seg_west_mash = Mesh::from(Cuboid::new(SIZE_T, SIZE_T, 1.0));
    commands.spawn((
      NotShadowCaster,
      NotShadowReceiver,
      Mesh3d(meshes.add(sky_seg_west_mash)),
      // MeshMaterial3d(materials.add(Color::WHITE)),
      MeshMaterial3d(
        materials.add(StandardMaterial {
          // base_color: Color::srgb(0.5, 0.8, 1.0), // Sky blue
          // base_color: Color::srgb(255.0, 0.0, 0.0), // Sky blue
          base_color_texture: Some(sky_seg_west.clone()),
          base_color: Color::from(BLUE_200), // Sky blue
          cull_mode: Some(Face::Front), // Render the inside of the sphere
          unlit: !false,
          double_sided: false,
          ..default()
        })
      ),
      Transform::from_xyz(0.0, OFFSET_T, -SIZE_T / 2.0),
      // Transform::from_rotation(Quat::from_rotation_x(3.141592 / 2.0)),
      // Quat::from_rotation_z(2f32),
      PhysicsStaticObject,
      AnyObject,
    ));
  }

  {
    let sky_seg_up: Handle<Image> = load_base_texture(
      &asset_server,
      EImagePaths::SkySegUp.as_str()
    );

    let sky_seg_top_mash = Mesh::from(Cuboid::new(SIZE_T, 1.0, SIZE_T));
    commands.spawn((
      NotShadowCaster,
      NotShadowReceiver,
      Mesh3d(meshes.add(sky_seg_top_mash)),
      // MeshMaterial3d(materials.add(Color::WHITE)),
      MeshMaterial3d(
        materials.add(StandardMaterial {
          // base_color: Color::srgb(0.5, 0.8, 1.0), // Sky blue
          // base_color: Color::srgb(255.0, 0.0, 0.0), // Sky blue
          base_color_texture: Some(sky_seg_up.clone()),
          base_color: Color::from(BLUE_200), // Sky blue
          cull_mode: Some(Face::Front), // Render the inside of the sphere
          unlit: !false,
          double_sided: false,
          ..default()
        })
      ),
      Transform::from_xyz(0.0, SIZE_T / 2.0, 0.0),
      // Transform::from_rotation(Quat::from_rotation_x(3.141592 / 2.0)),
      // Quat::from_rotation_z(2f32),
      PhysicsStaticObject,
      AnyObject,
    ));
  }

  // let mesh = Mesh::from(Sphere { radius: SIZE_T });
  // let mesh = Mesh::from(Cuboid::new(SIZE_T, SIZE_T, SIZE_T));
  // let mesh = Sphere::new(SIZE_T).mesh().uv(64, 64).into();
  // let mesh = Sphere::new(SIZE_T).mesh().uv(16, 16);
  // let mesh = Sphere::new(SIZE_T).mesh().ico(64).unwrap();

  // let shapes = [
  //   meshes.add(Cuboid::default()),
  //   meshes.add(Tetrahedron::default()),
  //   meshes.add(Capsule3d::default()),
  //   meshes.add(Torus::default()),
  //   meshes.add(Cylinder::default()),
  //   meshes.add(Cone::default()),
  //   meshes.add(ConicalFrustum::default()),
  //   meshes.add(Sphere::default().mesh().ico(5).unwrap()),
  //   meshes.add(Sphere::default().mesh().uv(32, 18)),
  // ];

  // commands.spawn((
  //   Mesh3d(meshes.add(mesh)),
  //   // MeshMaterial3d(materials.add(Color::WHITE)),
  //   MeshMaterial3d(
  //     materials.add(StandardMaterial {
  //       // base_color: Color::srgb(0.5, 0.8, 1.0), // Sky blue
  //       // base_color: Color::srgb(255.0, 0.0, 0.0), // Sky blue
  //       base_color_texture: Some(texture.clone()),
  //       base_color: Color::from(BLUE_200), // Sky blue
  //       cull_mode: Some(Face::Front), // Render the inside of the sphere
  //       unlit: true,
  //       ..default()
  //     })
  //   ),
  //   // Transform::from_xyz(0.0, 10.0, 0.0),
  //   Transform::from_rotation(Quat::from_rotation_x(3.141592 / 2.0)),
  //   // Quat::from_rotation_z(2f32),
  //   PhysicsStaticObject,
  //   AnyObject,
  // ));

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
  // mut sky: Query<&mut Transform, With<MSkyMarker>>
) {
  // let mut transform = sky.single_mut();
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
