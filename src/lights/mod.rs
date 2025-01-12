use bevy::{
  input::common_conditions::{ input_just_pressed, input_pressed },
  pbr::{
    CascadeShadowConfig,
    CascadeShadowConfigBuilder,
    DirectionalLightShadowMap,
    VolumetricLight,
  },
  prelude::*,
};
use light_consts::lux;

use crate::AnyObject;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MPointLightMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MDirLightMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MPointLightFromMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MPointLightToMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MLightsPlugin;

// prettier-ignore
impl Plugin for MLightsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, startup)
      .add_systems(Update, update)
      .add_systems(PostUpdate, (
        switch_light_illuminance, 
      ).run_if(input_pressed(KeyCode::KeyP)));
      // ).run_if(input_just_pressed(KeyCode::KeyP)));

    app
      .insert_resource(DirectionalLightShadowMap { 
        size: 1024/2, // 2248 == default
      });



    // app.insert_resource(AmbientLight {
    //   color: Color::default(),
    //   brightness: 500.0,
    // });
  }
}

const POS: Vec3 = Vec3::new(0.0, 10.0, 0.0);
// const DIR_LIGHT_POS_2: Vec3 = Vec3::new(10000.0, 5500.0, 10000.0);
const DIR_LIGHT_POS_2: Vec3 = Vec3::new(10000.0, 8500.0, 10000.0);

// prettier-ignore
fn startup(
  dir_light_shadow_map: Res<DirectionalLightShadowMap>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {
  
  // dbgln!("{:?}", dir_light_shadow_map);

  // const COUNT: usize = 10;
  // let position_range = -200.0..200.0;
  // let radius_range = 5.0..20.0;
  // let pos_len = position_range.end - position_range.start;
  // let radius_len = radius_range.end - radius_range.start;
  // let mesh = meshes.add(Sphere::new(1.0).mesh().uv(120, 64));

  // for i in 0..COUNT {
  //   let percent = (i as f32) / (COUNT as f32);
  //   let radius = radius_range.start + percent * radius_len;
  //   // sphere light
  //   commands
  //     .spawn((
  //       Mesh3d(mesh.clone()),
  //       MeshMaterial3d(
  //         materials.add(StandardMaterial {
  //           base_color: Color::srgba(0.5, 0.5, 1.0, 0.25),
  //           unlit: true,
  //           alpha_mode: AlphaMode::Blend,
  //           ..default()
  //         }),
  //       ),
  //       Transform::from_xyz(position_range.start + percent * pos_len, 200.0, 0.0).with_scale(
  //         Vec3::splat(radius)
  //       ),
  //       AnyObject
  //     ))
  //     .with_child(PointLight {
  //       intensity: 10000.0,
  //       radius,
  //       color: Color::srgb(0.2, 0.2, 1.0),
  //       ..default()
  //     });
  // }

  commands.insert_resource(AmbientLight {
    color: Color::default(),
    brightness: 80.0 * 2.0,
  });

  // commands.spawn(
  //   AmbientLight {
  //     color: Color::srgb_u8(255, 255, 255),
  //     ..default()
  //   });

  // commands.spawn((
  //   SpotLight {
  //     color: Color::srgb_u8(255, 255, 255),
  //     // intensity: 1.0,
  //     shadows_enabled: true,
  //     // shadow_depth_bias: 0.1,
  //     // shadow_normal_bias: 0.1,
  //     range: 256.0,
  //     radius: 50.0,
  //     ..default()
  //   },
  //   Transform::from_xyz(POS.x, POS.y, POS.z).looking_at(Vec3::ZERO, Vec3::ZERO),
  //   MPointLightMarker,
  // ));




  commands.spawn((
    DirectionalLight {
      // color: Color::srgb_u8(255, 255, 255),
      color: Color::default(),
      illuminance: 2750.0, // lux::FULL_DAYLIGHT,
      shadows_enabled: true,
      // shadow_depth_bias: 0.1,
      // shadow_normal_bias: 0.1,
      ..default()
    },
    Transform::from_xyz(
      DIR_LIGHT_POS_2.x, 
      DIR_LIGHT_POS_2.y, 
      DIR_LIGHT_POS_2.z
    ).looking_at(Vec3::ZERO, Vec3::ZERO),
    CascadeShadowConfigBuilder {
      num_cascades: 5,
      minimum_distance: 50.0,
      maximum_distance: 200000.0,
      // need to test out
      first_cascade_far_bound: 70.0,
      // overlap_proportion: 0.5,
      ..default()
    }.build(),
    // VolumetricLight,
    MDirLightMarker,
  ));
  
  
  // commands.spawn((
  //   PointLight {
  //     color: Color::srgb_u8(255, 255, 255),
  //     intensity: 4096.0 * 1000.0,
  //     range: 256.0,
  //     radius: 256.0,
  //     shadows_enabled: true,
  //     // intensity: 1.0,
  //     ..default()
  //   },
  //   Transform::from_xyz(
  //     DIR_LIGHT_POS_2.x, 
  //     DIR_LIGHT_POS_2.y, 
  //     DIR_LIGHT_POS_2.z
  //   ).looking_at(Vec3::ZERO, Vec3::ZERO),
  //   // Transform::from_xyz(POS.x, POS.y, POS.z).looking_at(Vec3::ZERO, Vec3::ZERO),
  //   // VolumetricLight,
  //   MPointLightMarker,
  // ));

  // commands.spawn((
  //   Transform::from_xyz(POS.x, POS.y, POS.z).looking_at(Vec3::ZERO, Vec3::ZERO),
  //   Mesh3d(meshes.add(Sphere::new(0.5))),
  //   MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
  //   MPointLightFromMarker,
  // ));

  // commands.spawn((
  //   Transform::from_xyz(POS.x, POS.y, POS.z).looking_at(Vec3::ZERO, Vec3::ZERO),
  //   Mesh3d(meshes.add(Sphere::new(0.5))),
  //   MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
  //   MPointLightToMarker,
  // ));

}

// prettier-ignore
fn switch_light_illuminance(
  mut query: Query<&mut DirectionalLight, With<MDirLightMarker>>, 
  time: Res<Time>
) {

  let mut m_dir_light = query.single_mut();
  m_dir_light.illuminance += (time.delta_secs_f64() as f32) * 100.0;

  dbgln!("{:?}", m_dir_light.illuminance);

  // for mut light in query.iter_mut() {
  //   // Example: Switch to twilight after 5 seconds
  //   // if time.delta_secs_f64() > 5.0 {
  //   //   light.illuminance = lux::AMBIENT_DAYLIGHT;
  //   // }
  //   if time.delta_secs_f64() > 5.0 {
  //     light.illuminance += time.delta_secs_f64() as f32;
  //   }
  // }
}

// prettier-ignore
fn update(
  mut query_dir_light: Query<&mut Transform, With<MDirLightMarker>>
  // mut query_point_light: Query<&mut Transform, With<MPointLightMarker>>
) {
  let mut transform = query_dir_light.single_mut();
  // let mut transform = query_point_light.single_mut();
  // transform.rotate_local_y(0.01);

//   match transform {
//     Transform::Perspective(persp) => {
//       // we have a perspective projection
//     }
//     Transform::Orthographic(ortho) => {
//       // we have an orthographic projection
//     }
//   }
}
