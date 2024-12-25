use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MPointLightMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MPointLightFromMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MPointLightToMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MLightsPlugin;

impl Plugin for MLightsPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, startup);
    app.add_systems(Update, update);
    // app.insert_resource(AmbientLight {
    //   color: Color::default(),
    //   brightness: 500.0,
    // });
  }
}

const POS: Vec3 = Vec3::new(0.0, 10.0, 0.0);
const POS_2: Vec3 = Vec3::new(10000.0, 5500.0, 10000.0);

// prettier-ignore
fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {
  

  commands.insert_resource(AmbientLight {
    color: Color::default(),
    brightness: 80.0,
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
    Transform::from_xyz(POS_2.x, POS_2.y, POS_2.z).looking_at(Vec3::ZERO, Vec3::ZERO),
    MPointLightMarker,
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
    mut query_point_light: Query<&mut Transform, With<MPointLightMarker>>
) {
  let mut transform = query_point_light.single_mut();
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
