use bevy::{
  pbr::{ CascadeShadowConfig, CascadeShadowConfigBuilder, DirectionalLightShadowMap },
  prelude::*,
};

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
      .add_systems(Update, update);

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
  
  dbgln!("{:?}", dir_light_shadow_map);


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
      illuminance: light_consts::lux::OVERCAST_DAY,
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
      // num_cascades: 1,
      // maximum_distance: 20.0,
      minimum_distance: 0.5,
      maximum_distance: 20000000.0,
      // need to test out
      // first_cascade_far_bound: 0.5,
      // overlap_proportion: 0.5,
      ..default()
    }.build(),
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
  //   Transform::from_xyz(POS.x, POS.y, POS.z).looking_at(Vec3::ZERO, Vec3::ZERO),
  //   MPointLightMarker,
  // ));

  commands.spawn((
    Transform::from_xyz(POS.x, POS.y, POS.z).looking_at(Vec3::ZERO, Vec3::ZERO),
    Mesh3d(meshes.add(Sphere::new(0.5))),
    MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
    MPointLightFromMarker,
  ));

  commands.spawn((
    Transform::from_xyz(POS.x, POS.y, POS.z).looking_at(Vec3::ZERO, Vec3::ZERO),
    Mesh3d(meshes.add(Sphere::new(0.5))),
    MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
    MPointLightToMarker,
  ));
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
