// use tracing::instrument;

use wgpu::{ Face, PrimitiveTopology };

// prettier-ignore
use avian3d::{
  parry::{na::Scale3}, 
  prelude::{
    collision::contact_reporting::{ Collision, CollisionEnded, CollisionStarted }, 
    AngularVelocity, CoefficientCombine, Collider, CollisionMargin, ExternalAngularImpulse, 
    ExternalForce, ExternalImpulse, ExternalTorque, LinearVelocity, LockedAxes, Mass, 
    MaxLinearSpeed, Restitution, RigidBody,
  }
};

// prettier-ignore
use bevy::{
  animation::transition, 
  asset::Handle, color::palettes::tailwind::*, 
  math::{Affine2, Vec2}, 
  // pbr::{FogVolume, OpaqueRendererMethod, VolumetricFog}, 
  prelude::{AlphaMode, Visibility}, 
  time::{Real, Time}
};

// prettier-ignore
use bevy::app::{ App, 
  FixedUpdate, Plugin, PostUpdate, Startup, Update
};

// prettier-ignore
use bevy::input::{
  common_conditions::{ input_just_pressed, input_just_released },
  keyboard::KeyboardInput,
  mouse::{ MouseButtonInput, MouseScrollUnit, MouseWheel },
  ButtonInput,
  ButtonState,
};

// prettier-ignore
use bevy::pbr::{ 
  wireframe::Wireframe, ExtendedMaterial, MaterialPlugin, MeshMaterial3d, 
  NotShadowCaster, NotShadowReceiver, StandardMaterial,
};

// prettier-ignore
use bevy::prelude::{ 
  in_state, BuildChildren, Camera3d, Capsule3d, ChildBuild, Commands, Component, 
  Cuboid, Deref, DerefMut, Entity, EventReader, IntoSystemConfigs, KeyCode, Mesh, 
  Mesh3d, MouseButton, Projection, Query, Res, ResMut, Resource, Sphere, State, 
  Transform, With, Without,
};

// prettier-ignore
use bevy::{
  asset::{ AssetServer, Assets },
  audio::{ AudioPlayer, PlaybackSettings },
  color::Color,
  core::Name,
  core_pipeline::prepass::{ DepthPrepass, NormalPrepass },
  gltf::GltfAssetLabel,
  math::{ Dir3, Vec3, VectorSpace },
  render::camera::PhysicalCameraParameters,
  scene::SceneRoot,
  text::cosmic_text::ttf_parser::Tag,
  utils::default,
};

// prettier-ignore
use bevy_render::{ 
  mesh::Indices, 
  render_resource::{ AsBindGroup, RenderPipeline, ShaderRef }, 
};

pub mod animation_ids;

// prettier-ignore
use crate::{
  dbgln, 
  debug::{ 
    get_defaul_physic_debug_params, is_allowed_debug_engine, 
    is_allowed_debug_fps, is_allowed_debug_physics 
  }, 
};

// prettier-ignore
use crate::{
  app_config::{self, debug::DebugConfig, *}, 
  asset_loader::audio_cache::{ cache_load_audio, AudioCache }, 
  camera::{get_player_camera, get_view_camera, PlayerCameraMarker}, 
  entities::with_children::MEntityBigSphere, 
  lights::{ MDirLightMarker, MPointLightFromMarker, MPointLightMarker, MPointLightToMarker }, 
  m_lib::physics, materials::cam_pos_1::CamPosExtension, 
  // post_processing_pipiline::test_example::CustomPostProcessSettings, 
  state::MGameState, 
  sys_paths, AnyObject, 
  COLLISION_MARGIN
};

use sys_paths::audio::EAudio;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct FullScreenShaderQuad;

// #[derive(Component, Debug, PartialEq, Eq)]
// pub struct PlayerCameraMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct PlayerMarker;

#[derive(Resource)]
struct CMaxLinearSpeedXZ(f32);

pub struct PlayerPlugin;

// static mut IS_LEFT_DOWW: Option<bool> = Some(false);
// static IS_LEFT_DOWW: Mutex<Option<bool>> = Mutex::new(Some(false));

#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

// prettier-ignore
impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, startup)

      // .add_systems(FixedUpdate,on_pause.run_if((
      //     in_state(MGameState::Paused)
      //   )
      // ))
      .add_systems(PostUpdate,
        (
          handle_bullet_out_of_allowed_area,
        )
      )
      .add_systems(FixedUpdate,
        (
          update,
          // control_cam,
          keyboard_events,
          // handle_drag,
          cam_track_object,
          cam_track_object_origin,
          detect_bullet_collision,
          // update_shader_quad_position,
          update_extended_material,
        ).run_if(in_state(MGameState::Running))
      )
      .add_systems(Update,(control_cam, zoom_on_scroll, handle_drag))
      // .add_systems(Update,update)
      // .add_systems(Update,zoom_on_scroll)
      // .add_systems(Update,keyboard_events)
      // .add_systems(Update,cam_track_object)
      // .add_systems(Update,cam_track_object_origin)
      // .add_systems(Update,detect_bullet_collision)
      .add_systems(Update,
        (
          handle_left_click
        )
          .run_if(in_state(MGameState::Running))
          .run_if(input_just_pressed(MouseButton::Left))

      )
      .add_systems(Update,
        (
          mk_jump,
        )
          .run_if(in_state(MGameState::Running))
          .run_if(input_just_pressed(KeyCode::Space))
      )
      // .add_systems(FixedUpdate,( 
      //     on_q_pressed.run_if(
      //       input_just_pressed(KeyCode::Space)
      //     ),
      //     on_m_left_down.run_if(
      //       input_just_pressed(MouseButton::Left)
      //     ),
      //     on_m_left_up.run_if(
      //       input_just_released(MouseButton::Left)
      //     ),
      //   )
      // )
      .add_systems(Update,constrain_linear_xz_speed)
      .insert_resource(CMaxLinearSpeedXZ(150.0))
      // .insert_resource(CMaxLinearSpeedXZ(30.0))

      .add_plugins((
        MaterialPlugin::<ExtendedMaterial<StandardMaterial, CamPosExtension>>::default()
      ));

    }
}

// fn update_camera_height(
//   camera_query: Query<&Transform, With<PlayerCameraMarker>>,
//   mut water_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, CamPosExtension>>>
// ) {
//   let x = water_materials.as_mut();

//   if let Ok(camera_transform) = camera_query.get_single() {
//     let height = camera_transform.translation.y;
//     for mut material in material_query.iter_mut() {
//       material.height = height;
//     }
//   }
// }

const BULLET_MIN_Y_ALLOWED: f32 = -10.0;
const MUL_POS: f32 = 5.0;
const POS: Vec3 = Vec3::new(-2.5 * MUL_POS, 30.5 * MUL_POS, 9.0 * MUL_POS);
// const POS: Vec3 = Vec3::new(-20.5 * MUL_POS, 30.5 * MUL_POS, -20.0 * MUL_POS);

fn update() {}

// fn test(commands: &mut Commands) -> Entity {
//   let id = commands.spawn(RigidBody::Dynamic).id();
//   id
// }
// prettier-ignore
// fn create_fullscreen_quad() -> Mesh {
//     let mut mesh = Mesh::from(Plane3d {

//     });

//     // Define the four vertices of the quad in normalized device coordinates (NDC)
//     let vertices = [
//         // Position        // UV coordinates
//         (Vec3::new(-1.0, -1.0, 0.0), Vec2::new(0.0, 0.0)), // Bottom-left
//         (Vec3::new( 1.0, -1.0, 0.0), Vec2::new(1.0, 0.0)), // Bottom-right
//         (Vec3::new( 1.0,  1.0, 0.0), Vec2::new(1.0, 1.0)), // Top-right
//         (Vec3::new(-1.0,  1.0, 0.0), Vec2::new(0.0, 1.0)), // Top-left
//     ];

//     let indices = [
//         0, 1, 2, 0, 2, 3, // Two triangles to form the quad
//     ];

//     mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.iter().map(|v| v.0).collect::<Vec<Vec3>>());
//     mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertices.iter().map(|v| v.1).collect::<Vec<Vec2>>());
//     mesh.set_indices(Some(Indices::U32(indices.to_vec())));

//     mesh
// }

// https://bevyengine.org/examples/shaders/custom-post-processing/

// fn update_camera_position(
//     camera_query: Query<&Transform, With<Camera>>,
//     // mut pipeline_query: Query<&mut RenderPipeline>,
// ) {
//     // if let Ok(camera_transform) = camera_query.single() {
//     //     let camera_position = camera_transform.translation;

//     //     // Update shader with the new camera position
//     //     for mut pipeline in pipeline_query.iter_mut() {
//     //         pipeline.set_uniform("camera_position", camera_position);
//     //     }
//     // }
// }

// use bevy::prelude::*;
// use bevy::render::render_resource::{ Shader, SpecializedMeshPipeline };
// use bevy::render::mesh::{ Mesh, VertexAttributeValues };
// use bevy::asset::Handle;

// #[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
// struct CameraPositionMaterial {
//   pub camera_position: Vec3,
// }

fn update_extended_material(
  mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, CamPosExtension>>>,
  // material_query: Query<&Handle<CamPosExtension>>,
  camera_query: Query<&Transform, With<PlayerMarker>>,
  time: Res<Time<Real>>
) {

  let mut mat = materials.as_mut();
  let trans = camera_query.get_single().unwrap();

  mat.iter_mut().for_each(|(k, v)| {
    // dbgln!("trans.translation.y: {}", trans.translation.y);
    v.extension.height = trans.translation.y;
    v.extension.time_t += (time.delta_secs_f64() as f32); //  /100.0;
    // dbgln!("v.extension.time_t: {}", v.extension.time_t); 
    // v.extension.some_value = 10.0;
    // v.base.base_color = Color::srgba_u8(70, 70, 180, 5);
  });

}

// prettier-ignore
fn startup(
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
  // mut water_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, CamPosExtension>>>
) {
  // let id = test(&mut commands);

  // commands.spawn((
  //   SceneRoot(
  //     asset_server.load(
  //       GltfAssetLabel::Scene(0).from_asset("characters/erica/erika-base.reexported-3-0-deg.glb")
  //       // GltfAssetLabel::Scene(0).from_asset("characters/erica/erika-base.glb"),
  //       // GltfAssetLabel::Scene(0).from_asset("characters/erica/erika-base.reexported.glb"),
  //     )
  //   ),
  //   // Transform::from_xyz(POS.x, POS.y + 5.0, POS.z), // .looking_at(POS, Vec3::Y),
  //   // Transform::from_scale(Vec3::new(2.0, 2.0, 2.0)),
  //   Transform {
  //     // translation: Vec3::new(POS.x, POS.y, POS.z),
  //     translation: Vec3::new(POS.x, 30.0, POS.z),
  //     scale: Vec3::new(3.0, 3.0, 3.0),
  //     ..Default::default()
  //   },
  // ));

  // commands.spawn((
  //   SceneRoot(
  //     asset_server.load(
  //       GltfAssetLabel::Scene(0).from_asset("characters/erica/erika-base.reexported-3-0-deg.glb")
  //       // GltfAssetLabel::Scene(0).from_asset("characters/erica/erika-base.glb"),
  //       // GltfAssetLabel::Scene(0).from_asset("characters/erica/erika-base.reexported.glb"),
  //     )
  //   ),
  //   Transform {
  //     // translation: Vec3::new(POS.x, POS.y, POS.z),
  //     // translation: Vec3::new(0.0, 0.0, 0.0),
  //     translation: Vec3::new(POS.x, 30.0, POS.z),
  //     scale: Vec3::new(1.0, 1.0, 1.0),
  //     ..Default::default()
  //   },
  // ));

  // let water_base_material: StandardMaterial = StandardMaterial {
  //   unlit: !false,
  //   // double_sided: true,
  //   cull_mode: Some(Face::Front),
  //   base_color: Color::srgba_u8(255, 40, 40, 20),
  //   // base_color: Color::srgba_u8(255, 255, 255, 255),
  //   opaque_render_method: OpaqueRendererMethod::Auto,
  //   // alpha_mode: AlphaMode::Blend,
  //   ..default()
  // };
  // let water_material_handle = materials.add(water_base_material);

  // let water_material_handle = water_materials.add(ExtendedMaterial {
  //   base: water_base_material,
  //   extension: CamPosExtension {
  //     height: 0.0,
  //     time_t: 0.0,
  //   },
  // });

  commands.spawn(get_view_camera());

  commands
    .spawn((
      RigidBody::Dynamic,
      CollisionMargin(COLLISION_MARGIN * 1.0),
      Collider::capsule(2.0, 100.0),
      // Restitution::new(0.0),
      Restitution {
        coefficient: 0.0,
        combine_rule: CoefficientCombine::Min,
      },
      Transform::from_xyz(POS.x, POS.y, POS.z), // .looking_at(POS, Vec3::Y),
      // Mesh3d(meshes.add(Capsule3d::new(2.0, 5.0))),
      // MeshMaterial3d(materials.add(Color::srgb_u8(127, 255, 0))),
      Mass(100.0),
      Visibility::default(),
      LockedAxes::ROTATION_LOCKED,
      // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
      // MaxLinearSpeed(5.0),
      PlayerMarker,
      Name::new("p_player_t"),
    ))
    // .with_children(|children| {
    //   children.spawn((
    //     SceneRoot(
    //       asset_server.load(
    //         GltfAssetLabel::Scene(0).from_asset(
    //           "characters/erica/erika-base.reexported-3-0-deg.glb"
    //         )
    //         // GltfAssetLabel::Scene(0).from_asset("characters/erica/erika-base.glb"),
    //         // GltfAssetLabel::Scene(0).from_asset("characters/erica/erika-base.reexported.glb"),
    //       )
    //     ),
    //     Transform {
    //       // translation: Vec3::new(POS.x, POS.y, POS.z),
    //       translation: Vec3::new(0.0, 0.0, 0.0),
    //       scale: Vec3::new(1.0, 1.0, 1.0),
    //       ..Default::default()
    //     },
    //   ));
    // })
    .with_children(|children| {
      // children.spawn(get_view_camera());
      children.spawn(get_player_camera())
        .with_children(|parent| {
          parent.spawn((
            Transform::from_xyz(0.0, -1.0, 0.0), // .looking_at(POS, Vec3::Y),
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 10.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(255, 40, 40))),
            NotShadowCaster,
            NotShadowReceiver,
          ));
          // parent.spawn((
          //   Transform::from_xyz(0.0, 0.0, -1.0), // .looking_at(POS, Vec3::Y),
          //   Mesh3d(meshes.add(Cuboid::new(3.0, 3.0, 0.1))),
          //   MeshMaterial3d(water_material_handle),
          //   // MeshMaterial3d(materials.add(Color::srgba_u8(255, 40, 40, 30))),
          //   // AnyObject,
          //   NotShadowCaster,
          //   NotShadowReceiver,
          // ));
        });
      });
      // .insert(VolumetricFog {
      //   // This value is explicitly set to 0 since we have no environment map light
      //   ambient_intensity: 0.5,
      //   ..default()
      // });

    // Add the fog volume.
    // commands.spawn((
    //   FogVolume::default(),
    //   Transform::from_scale(Vec3::splat(20_000.0)),
    // ));



}

// fn accelerate_bodies(mut query: Query<(&mut LinearVelocity, &mut AngularVelocity)>) {
//   for (mut linear_velocity, mut angular_velocity) in query.iter_mut() {
//     linear_velocity.x += 0.05;
//     angular_velocity.z += 0.05;
//   }
// }

// fn update_shader_quad_position(
//   camera_query: Query<&Transform, (With<PlayerMarker>, Without<FullScreenShaderQuad>)>,
//   mut quad_query: Query<&mut Transform, (With<FullScreenShaderQuad>, Without<PlayerMarker>)>
// ) {

//   let camera_transform = camera_query.single();

//   // Get the camera's position
//   let camera_position = camera_transform.translation;
//   dbgln!("Camera position: {:?}", camera_position);

//   // Update the position of the quad to be in front of the camera
//   for mut quad_transform in quad_query.iter_mut() {
//     // Position the quad directly in front of the camera at a fixed distance (e.g., 5 units)
//     let quad_position = camera_position + camera_transform.forward() * 5.0; // 5.0 units in front of the camera

//     quad_transform.translation = quad_position;

//     // Optionally, make the quad always face the camera (if needed)
//     quad_transform.rotation = Quat::from_rotation_y(camera_transform.rotation.y);
//   }
// }

// prettier-ignore
fn cam_track_object(
  // query_big_sphere: Query<&Transform, (With<MEntityBigSphere>, Without<MPointLightMarker>)>,
  // mut query_point_light: Query<&mut Transform, (With<MPointLightMarker>, Without<MEntityBigSphere>)>,
  // mut query_camera: Query<&mut Transform, (With<PlayerCameraMarker>, Without<MPointLightMarker>, Without<MEntityBigSphere>)>
) {

  // let trans_sphere = query_big_sphere.single();
  // let mut trans_light = query_point_light.single_mut();
  // // trans_light.rotate_local_y(0.01);
  
  // trans_light.translation.x = trans_sphere.translation.x;
  // trans_light.translation.y = trans_sphere.translation.y + 50.0;
  // trans_light.translation.z = trans_sphere.translation.z;
  // trans_light.look_at(trans_sphere.translation, Vec3::Z);
  // // let mut trans_cam = query_camera.single_mut();
  // // trans_cam.look_at(trans_sphere.translation, Vec3::Y);
  // // trans_cam.translation.x = trans_sphere.translation.x + 5.0;
  // // trans_cam.translation.y = trans_sphere.translation.y + 5.0;
  // // trans_cam.translation.z = trans_sphere.translation.z + 5.0;
}

// prettier-ignore
fn cam_track_object_origin(
  // query_big_sphere: Query<&Transform, (
  //   With<MEntityBigSphere>, 
  //   Without<MPointLightMarker>, 
  //   Without<MPointLightFromMarker>, 
  //   Without<MPointLightToMarker>
  // )>,
  // query_point_light: Query<&Transform, (
  //   With<MPointLightMarker>, 
  //   Without<MEntityBigSphere>,
  //   Without<MPointLightFromMarker>, 
  //   Without<MPointLightToMarker>,
  // )>,
  // mut from: Query<&mut Transform, (
  //   With<MPointLightFromMarker>, 
  //   Without<MEntityBigSphere>, 
  //   Without<MPointLightMarker>, 
  //   Without<MPointLightToMarker>
  // )>,
  // mut to: Query<&mut Transform, (
  //   With<MPointLightToMarker>, 
  //   Without<MEntityBigSphere>, 
  //   Without<MPointLightMarker>,
  //   Without<MPointLightFromMarker>, 
  // )>,
) {

  // return;

  // let trans_sphere = query_big_sphere.single();
  // let trans_light = query_point_light.single();

  // // trans_light.rotate_local_y(0.01);

  // let mut m_from = from.single_mut();
  // m_from.translation = trans_light.translation.clone();
  // m_from.translation.y -= 2.5;

  // let mut m_to = to.single_mut();
  // m_to.translation = trans_sphere.translation.clone();
  // m_to.translation.y += 2.5;

  // // MPointLightFromMarker;
  // // MPointLightToMarker;

  // // let trans_sphere = query_big_sphere.single();
  // //   let mut trans_light = query_point_light.single_mut();
  // // // trans_light.rotate_local_y(0.01);
  
  // // trans_light.translation.x = trans_sphere.translation.x;
  // // trans_light.translation.y = trans_sphere.translation.y + 20.0;
  // // trans_light.translation.z = trans_sphere.translation.z;
  // // trans_light.look_at(trans_sphere.translation, Vec3::ZERO);
  // // // let mut trans_cam = query_camera.single_mut();
  // // // trans_cam.look_at(trans_sphere.translation, Vec3::Y);
  // // // trans_cam.translation.x = trans_sphere.translation.x + 5.0;
  // // // trans_cam.translation.y = trans_sphere.translation.y + 5.0;
  // // // trans_cam.translation.z = trans_sphere.translation.z + 5.0;
}

// #[tracing::instrument]
fn zoom_on_scroll(
  mut mw_evt: EventReader<MouseWheel>,
  mut query_camera: Query<&mut Projection, With<PlayerCameraMarker>>
) {
  let Projection::Perspective(persp): &mut Projection = query_camera
    .single_mut()
    .into_inner() else {
    return;
  };

  for mouse_wheel_event in mw_evt.read() {
    let (_dx, _dy) = match mouse_wheel_event.unit {
      // MouseScrollUnit::Line => (mouse_wheel_event.x, mouse_wheel_event.y),
      // MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
      MouseScrollUnit::Line => (mouse_wheel_event.x, mouse_wheel_event.y),
      MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
    };

    // dbgln!("Mouse wheel: X: {}, Y: {}", dx, dy);

    let dy = _dy * -1.0;
    let val: f32 = persp.fov + dy / 30.0;

    // dbgln!("FOV: {}", val);

    if val < 0.05 {
      return;
    } else if val > 2.75 {
      return;
    }

    persp.fov = val;
    return;
  }
}

fn on_q_pressed() {
  // dbgln!("Q pressed");
}

// fn set_global_state(value: bool) {
//   let mut state = IS_LEFT_DOWW.lock().unwrap();
//   *state = Some(value);
// }

// fn get_global_state() -> Option<bool> {
//   let state = IS_LEFT_DOWW.lock().unwrap();
//   *state
// }

// prettier-ignore
fn on_m_left_down() { 
  // set_global_state(true); 
}
// prettier-ignore
fn on_m_left_up() { 
  // set_global_state(false); 
}

// prettier-ignore
// fn get_camera_direction(
//   query: Query<&GlobalTransform, With<Camera>>,
// ) {
//   if let Some(camera_transform) = query.iter().next() {
//     let forward = camera_transform.forward();
//     camera_transform.affine().translation.x;
//     dbgln!("Camera forward direction: {:?}", forward);
//   }
// }

// #[tracing::instrument]
// prettier-ignore
// fn abc(
//   mut commands: Commands,
//   mut query: Single<&mut PlayerMarker, (With<PlayerMarker>)>,
//   mut cam_parent_2: Single<&mut PlayerMarker, With<PlayerMarker>>
// ) {
//   let( mut cam_parent) = query.into_inner();
//   // cam_parent
//   let p = cam_parent.into_inner();
//   // commands.entity(cam_parent)
// }

// prettier-ignore
fn mk_jump(
  mut commands: Commands,
  // query: Query<(Entity, &RigidBody, &Transform), With<PlayerMarker>>
  query: Query<(Entity, &RigidBody), With<PlayerMarker>>
) {

  // return;

  // if let Some(is_left_m_btn_down) = get_global_state() {
  //   if !is_left_m_btn_down { return; }
  // }

  // let a: ExternalForce;
  // let b: ExternalTorque;
  // let c: ExternalImpulse;
  // let d: ExternalAngularImpulse;

  // ExternalForce::new(Vec3::new(0.1, 0.1, 0.1)).with_persistence(false);
  // let force = Vec3::new(10.0, 0.0, 0.0); // Replace with your desired force vector

  return;

  let mut force = ExternalImpulse::default();
  force
    .apply_impulse_at_point(
      Vec3::Y * 1.0 * 400.0, 
      Vec3::ZERO, 
      Vec3::ZERO
    )
    .with_persistence(false);

  // force
  //   .apply_impulse_at_point(
  //     Vec3::new(0.0, 50.0, 0.0),
  //     Vec3::new(0.0, 0.0, 0.0),
  //     Vec3::new(0.0, 0.0, 0.0)
  //   )
  //   .with_persistence(true);

  // let mut force = ExternalForce::default();
  // force.apply_force_at_point(
  //   Vec3::Y, 
  //   Vec3::X, 
  //   Vec3::ZERO
  // ).with_persistence(false);

  // let (entity, _rigid_body) = query.iter();
  let (entity, body) = query.single();
  commands
    .entity(entity)
    // .insert((RigidBody::Dynamic, force));
    .insert((RigidBody::Dynamic, force));
    // .insert(Force::new(force, transform.translation));

  // // for (entity, _rigid_body, transform) in query.iter() {
  // for (entity, _rigid_body) in query.iter() {
  //   dbgln!("Entity apply_impulse_at_point: entity: {:?}, force: {:?}", entity, force);
  //   commands
  //     .entity(entity)
  //     // .insert((RigidBody::Dynamic, force));
  //     .insert((RigidBody::Dynamic, force));
  //     // .insert(Force::new(force, transform.translation));
  // }
}

fn constrain_linear_xz_speed(
  mut q_lin_velocity: Query<&mut LinearVelocity, With<PlayerMarker>>,
  c_max_speed: Res<CMaxLinearSpeedXZ>
) {
  for mut velocity in q_lin_velocity.iter_mut() {
    let xz_speed = (velocity.x.powi(2) + velocity.z.powi(2)).sqrt();
    if xz_speed > c_max_speed.0 {
      let scale = c_max_speed.0 / xz_speed;
      velocity.x *= scale;
      velocity.z *= scale;
    }
  }
}

fn control_cam(
  g_state: Res<State<MGameState>>,
  mut q_lin_velocity: Query<&mut LinearVelocity, With<PlayerMarker>>,
  mut commands: Commands,
  mut mw_evt: EventReader<MouseWheel>,
  keys: Res<ButtonInput<KeyCode>>,
  mut q_camera: Query<&Transform, (With<PlayerCameraMarker>, Without<PlayerMarker>)>,
  // mut q_camera_parent: Query<&mut Transform, (With<PlayerMarker>, Without<PlayerCameraMarker>)>
  // mut q_camera_parent_2: Query<
  //   &mut Transform,
  //   (With<PlayerMarker>, Without<PlayerCameraMarker>)
  // >
  mut q_camera_parent: Query<
    (Entity, &mut RigidBody, &mut Transform),
    (With<PlayerMarker>, Without<PlayerCameraMarker>)
  >
  // query: Query<(Entity, &RigidBody, &Transform), With<PlayerMarker>>
) {
  // if let Some(is_left_m_btn_down) = get_global_state() {
  //   if !is_left_m_btn_down { return; }
  // }

  let extara_down = -10.0;
  let f = -10.0 - 2.0 - extara_down;
  let df = 1000.0;
  let mut impulse3 = Vec3::new(0.0, 0.0, 0.0);
  let mut apply_force = false;
  let mut is_in_water = false;

  let (entity, body, mut transform) = q_camera_parent.single_mut();
  if transform.translation.y < -0.0 {
    is_in_water = true;
  }

  if transform.translation.y < -f {
    // dbgln!("transform.translation.y: y {:.4}", transform.translation.y);

    if transform.translation.y < -1.75 {
      commands.entity(entity).insert(physics::get_gravity_scale(0.0));
      // commands.entity(entity).insert(physics::get_gravity_scale(0.06));
      // commands.entity(entity).insert(physics::get_gravity_scale(0.1));

      apply_force = true;
      let diff = transform.translation.y; //  - f;
      let abs_diff = (if diff < -df { -df } else { diff }).abs();
      let impulse = (abs_diff / df) * 20.0;
      let force = (abs_diff / df) * 10.0;
      let inverse = (diff / df).abs();

      // dbgln!("diff: y {:.4}", diff);

      let mut vel = q_lin_velocity.single_mut();
      vel.0 *= 0.99;
      // vel.0 *= 0.95;

      // commands
      //   .entity(entity)
      //   .insert(LinearVelocity(Vec3::ZERO))
      //   .insert(AngularVelocity(Vec3::ZERO));

      // dbgln!("transform.translation.y: y {:.4}, inverse: {:.4}", inverse, transform.translation.y);
      // XXX
      // dbgln!(
      //   "transform: y {:.4}, diff: {:.4}, impulse: {:.4}, force: {:.4}",
      //   transform.translation.y,
      //   diff,
      //   impulse,
      //   force
      // );

      // XXX

      impulse3.y = physics::get_gravity() * impulse;
      // impulse3.y = physics::get_gravity() * force;

      // if transform.translation.y < -8.0 {
      //   transform.translation.y += inverse * 100.0;
      //   // impulse3.y *= 2.0;
      // }
    } else {
      commands.entity(entity).insert(physics::get_gravity_scale(1.0));
    }
  } else {
    commands.entity(entity).insert(physics::get_gravity_scale(1.0));
  }

  if
    !keys.pressed(KeyCode::KeyW) &&
    !keys.pressed(KeyCode::KeyS) &&
    !keys.pressed(KeyCode::KeyA) &&
    !keys.pressed(KeyCode::KeyD) &&
    !keys.pressed(KeyCode::Space)
  {
    // if apply_force {
    let impulse = physics::get_external_impulse(impulse3, false);
    commands.entity(entity).insert((RigidBody::Dynamic, impulse));
    // }
    // let force = physics::get_external_force(impulse3, false);
    // commands.entity(entity).insert((RigidBody::Dynamic, force));
    return;
  }

  // let mut force = ExternalImpulse::default();
  // force
  //   .apply_impulse_at_point(
  //     Vec3::Y * 1.0 * 10.0,
  //     Vec3::X,
  //     Vec3::ZERO
  //   )
  //   .with_persistence(false);

  // // let (entity, _rigid_body) = query.iter();
  // let (entity, body) = query.single();
  // commands
  //   .entity(entity)
  //   // .insert((RigidBody::Dynamic, force));
  //   .insert((RigidBody::Dynamic, force));
  //   // .insert(Force::new(force, transform.translation));

  let mut l_max_speed: f32 = 10.0;
  let mut running_speed: f32 = 10.0; // 1.0;
  let mut jump_force: f32 = 0.0;
  let use_physics: bool = true;

  if !is_in_water && keys.pressed(KeyCode::KeyQ) {
    // running_speed = 2.0;
    // l_max_speed *= 3.0; // normal (prod);
    // l_max_speed *= 30.0; // debug (dev);
    l_max_speed *= 100.0; // debug (dev);
  }

  if !is_in_water && keys.pressed(KeyCode::Space) {
    jump_force = 500.0;
  }

  let m_state: &MGameState = g_state.get();

  let is_paused: bool = m_state == &MGameState::Paused;

  let force_scale_mul: f32 = 100.0 * running_speed;
  const FW_DIV_SCALE: f32 = 20.0;
  const LR_DIV_SCALE: f32 = 1.0;
  const BOOST_SPEED: f32 = 0.5;

  let fw: Dir3 = transform.forward();
  let mut x: f32 = ((fw.x / FW_DIV_SCALE) as f64).clamp(-1.0, 1.0) as f32;
  let mut y: f32 = ((fw.y / FW_DIV_SCALE) as f64).clamp(-1.0, 1.0) as f32;
  let mut z: f32 = ((fw.z / FW_DIV_SCALE) as f64).clamp(-1.0, 1.0) as f32;

  // assert!( x.abs() <= 1.0 && y.abs() <= 1.0 && z.abs() <= 1.0, "FW/BW: (x: {:.6}, y: {:.6}, z: {:.6})", x, y, z );

  impulse3.y += jump_force;

  if use_physics && !is_paused {
    if keys.pressed(KeyCode::KeyW) {
      impulse3 += Vec3::new(x, y, z) * force_scale_mul * 20.0 * BOOST_SPEED;
    } else if keys.pressed(KeyCode::KeyS) {
      impulse3 += Vec3::new(x * -1.0, y * -1.0, z * -1.0) * 20.0 * force_scale_mul * BOOST_SPEED;
    }

    let right = transform.right();
    x = (((x - right.x) / LR_DIV_SCALE) as f64).clamp(-1.0, 1.0) as f32;
    y = (((y - right.y) / LR_DIV_SCALE) as f64).clamp(-1.0, 1.0) as f32;
    z = (((z - right.z) / LR_DIV_SCALE) as f64).clamp(-1.0, 1.0) as f32;

    if keys.pressed(KeyCode::KeyA) {
      impulse3 += Vec3::new(x, y, z) * force_scale_mul * BOOST_SPEED;
    } else if keys.pressed(KeyCode::KeyD) {
      impulse3 += Vec3::new(x * -1.0, y * -1.0, z * -1.0) * force_scale_mul * BOOST_SPEED;
    }
  } else {
    // dbgln!("Camera is paused");
    if keys.pressed(KeyCode::KeyW) {
      transform.translation.x += x * (100.0 + l_max_speed / 5.0);
      transform.translation.y += y * (100.0 + l_max_speed / 5.0);
      transform.translation.z += z * (100.0 + l_max_speed / 5.0);
    } else if keys.pressed(KeyCode::KeyS) {
      transform.translation.x -= x * (100.0 + l_max_speed / 5.0);
      transform.translation.y -= y * (100.0 + l_max_speed / 5.0);
      transform.translation.z -= z * (100.0 + l_max_speed / 5.0);
    }

    let right: Dir3 = transform.right();
    x = (((x - right.x) / LR_DIV_SCALE) as f64).clamp(-1.0, 1.0) as f32;
    y = (((y - right.y) / LR_DIV_SCALE) as f64).clamp(-1.0, 1.0) as f32;
    z = (((z - right.z) / 20.0) as f64).clamp(-1.0, 1.0) as f32;

    if keys.pressed(KeyCode::KeyA) {
      transform.translation.x += x * 20.0;
      transform.translation.y += y * 20.0;
      transform.translation.z += z * 20.0;
    } else if keys.pressed(KeyCode::KeyD) {
      transform.translation.x -= x * 20.0;
      transform.translation.y -= y * 20.0;
      transform.translation.z -= z * 20.0;
    }
  }

  let force: ExternalImpulse = physics::get_external_impulse(impulse3, false);
  commands.entity(entity).insert((RigidBody::Dynamic, force));

  for mut velocity in q_lin_velocity.iter_mut() {
    let xz_speed = (velocity.x.powi(2) + velocity.z.powi(2)).sqrt();
    if xz_speed > l_max_speed {
      let scale = l_max_speed / xz_speed;
      velocity.x *= scale;
      velocity.z *= scale;
    }
  }
}

// fn process_bullets(
//   mut commands: Commands,
//   // query: Query<(Entity, &RigidBody, &Transform), With<PlayerMarker>>
//   q_bullet: Query<(Entity, &RigidBody), With<BulletMarker>>
// ) {
//   for (entity, rb_bullet) in q_bullet.iter() {
//     if rb_bullet.is_dynamic() {
//       dbgln!("Entity : entity: {:?}, bullet: {:?}", entity, rb_bullet);
//       commands.entity(entity).despawn();
//     }
//   }
// }

// fn detect_collisions(mut collision_events: EventReader<CollisionEvent>) {
//   for event in collision_events.iter() {
//     match event {
//       CollisionEvent::Started(entity1, entity2) => {
//         dbgln!("Collision started between {:?} and {:?}", entity1, entity2);
//       }
//       CollisionEvent::Stopped(entity1, entity2) => {
//         dbgln!("Collision stopped between {:?} and {:?}", entity1, entity2);
//       }
//     }
//   }
// }

// prettier-ignore
fn detect_bullet_collision(
  mut commands: Commands,
  query: Query<&Name>,
  mut collision_event_reader: EventReader<Collision>
) {
  for Collision(contacts) in collision_event_reader.read() {
    // if contacts.collision_stopped() {
    //   let type_t_1 = query.get(contacts.entity1).unwrap_or(&Name::new("unknown_t")).to_string();
    //   let type_t_2 = query.get(contacts.entity2).unwrap_or(&Name::new("unknown_t")).to_string();
    //   // let type_t_1 = query.get(contacts.body_entity1.unwrap()).unwrap_or(&Name::new("unknown_t")).to_string();
    //   // let type_t_2 = query.get(contacts.body_entity2.unwrap()).unwrap_or(&Name::new("unknown_t")).to_string();
    //   dbgln!(" end > : (type(1): {type_t_1}) collided (type(2): {type_t_2})");
    // }

    if contacts.collision_started() {
      // let type_t_1 = query
      //   .get(contacts.entity1)
      //   .map(|n| n.to_string())
      //   .unwrap_or("unknown_t".to_string());

      let type_t_1 = query.get(contacts.entity1).unwrap_or(&Name::new("unknown_t")).to_string();
      let type_t_2 = query.get(contacts.entity2).unwrap_or(&Name::new("unknown_t")).to_string();

      // let type_t_1 = query
      //   .get(contacts.body_entity1.unwrap())
      //   .unwrap_or(&Name::new("unknown_t"))
      //   .to_string();
      // let type_t_2 = query
      //   .get(contacts.body_entity2.unwrap())
      //   .unwrap_or(&Name::new("unknown_t"))
      //   .to_string();

      // dbgln!(" start > : (type(1): {type_t_1}) collided (type(2): {type_t_2})");

      if type_t_1 == "unknown_t" || type_t_2 == "unknown_t" {
        return;
      }

      if type_t_2 == type_t_1 {
        return;
      }

      if
        (type_t_1 == "p_bullet_t" && type_t_2 == "p_player_t") ||
        (type_t_1 == "p_player_t" && type_t_2 == "p_bullet_t")
      {
        return;
      }

      // dbgln!(
      //   "(Entities (name: {type_t_1} => {}) and (name: {type_t_2} => {}) are colliding), (bodies: {:?} and {:?} ) is_sensor: {:?}, collision_started: {:?}",
      //   contacts.entity1,
      //   contacts.entity2,
      //   contacts.body_entity1,
      //   contacts.body_entity2,
      //   contacts.is_sensor,
      //   contacts.collision_started()
      // );
      // dbgln!(
      //   "0: (type(1): {type_t_1}) collided (type(2): {type_t_2}) ({:?} => {:?})",
      //   contacts.body_entity1,
      //   contacts.body_entity2
      // );

      // return;

      if type_t_1 == "p_bullet_t" {
        dbgln!(
          "0: (type(1): {type_t_1}) collided (type(2): {type_t_2}) ({} => {})",
          contacts.entity1,
          contacts.entity2
        );
        commands.entity(contacts.entity1).despawn();
      }

      if type_t_2 == "p_bullet_t" {
        dbgln!(
          "1: (type(1): {type_t_1}) collided (type(2): {type_t_2}) ({} => {})",
          contacts.entity1,
          contacts.entity2
        );
        commands.entity(contacts.entity2).despawn();
      }
    }
  }
}

#[derive(Component, Debug, PartialEq, Eq)]
pub struct BulletMarker {
  name: String,
}

fn handle_bullet_out_of_allowed_area(
  mut commands: Commands,
  q_name: Query<&Name>,
  // query: Query<(Entity, &RigidBody, &Transform), With<PlayerMarker>>
  q_bullets: Query<(Entity, &Transform), With<BulletMarker>>
) {
  for (entity, transform) in q_bullets.iter() {
    if transform.translation.y < BULLET_MIN_Y_ALLOWED {
      let bullet_t = q_name.get(entity).unwrap_or(&Name::new("unknown_t")).to_string();
      // dbgln!("Bullet out of allowed area: {:?}", bullet_t);
      commands.entity(entity).despawn();
    }
  }
}

// prettier-ignore
fn handle_left_click(
  mut res_mut_audio_cache: Option<ResMut</*res_mut_texture_cache::*/AudioCache>>,
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut ev_m_motion: EventReader<bevy::input::mouse::MouseMotion>,
  mut ev_b_input: EventReader<MouseButtonInput>,
  mut player: Query<&mut Transform, (With<PlayerMarker>, Without<PlayerCameraMarker>)>,
  mut query_camera: Query<&mut Transform, (With<PlayerCameraMarker>, Without<PlayerMarker>)>
) {
  for ev_b in ev_b_input.read() {
    if ev_b.button == MouseButton::Left {

      let mut transform_parent = player.single_mut();
      let mut transform = query_camera.single_mut();
      let vec3_parent = transform_parent.translation;
      let fw_parent = Vec3::from(transform_parent.forward());
      let up_child = Vec3::from(transform.up());
      let mut to = Vec3::new( fw_parent.x,  up_child.z * 1.5, fw_parent.z);
      // dbgln!("to-vec-y: {} => up_child.z {}", transform.translation.y, up_child.z);
      
      let mut force = ExternalImpulse::default();
      force
        .apply_impulse_at_point(
          to * 2500.0, 
          Vec3::ZERO, 
          Vec3::ZERO
        )
        .with_persistence(false);

      let off_xz = 5.0;
      let norm_vec_3 = fw_parent.clone().normalize();

      let handle_bullet = (
        RigidBody::Dynamic,
        Collider::sphere(0.25),
        CollisionMargin(COLLISION_MARGIN * 1.0),
        // SweptCcd::default(),
        // old version
        // RigidBody {
        //   ccd: SweptCcd::default(),
        //   ..Default::default()
        // },

        Transform::from_xyz(
          vec3_parent.x + (norm_vec_3.x * off_xz), 
          vec3_parent.y +2.0, 
          vec3_parent.z + (norm_vec_3.z * off_xz)
        ), // .looking_at(Vec3::ZERO, Vec3::Y),
        Mesh3d(meshes.add(Sphere::new(0.25))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
        // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        force,
        Mass(10.0),
        get_defaul_physic_debug_params(),
        AnyObject,
        // Wireframe::default()
        BulletMarker { name: "bullet_marker".to_string() },
        Name::new("p_bullet_t"),
        // if is_allowed_debug_engine() { Wireframe } else { Wireframe::default() }, 
      );

      commands.spawn(handle_bullet);
      // commands.entity(object).insert(handle_bullet);
      // commands.entity(object).insert(Wireframe);

      // // dbgln!("Right mouse button pressed");
      // for ev_m in ev_m_motion.read() {
      //   // dbgln!("Mouse drag: X: {} px, Y: {} px", ev_m.delta.x, ev_m.delta.y);
      //   transform.rotate_local_y(ev_m.delta.x / 1000.0);
      //   transform.rotate_local_x((ev_m.delta.y / 1000.0) * 1.0);
      // }

      let audio_hashmap: &mut ResMut<AudioCache> = res_mut_audio_cache.as_mut().unwrap();

      let sound = cache_load_audio(
        audio_hashmap, 
        &asset_server, 
        EAudio::PaintballShoot.as_str(),
        false
      );

      // let sound: Handle<AudioSource> = asset_server.load(sys_paths::sounds::EPaths::PaintballShoot.as_str());
      
      commands.spawn((
        // AudioPlayer(soundtrack_player.track_list.first().unwrap().clone()),
        AudioPlayer(sound),
        // AudioPlayer(track_list.first().unwrap().clone()),
        PlaybackSettings {
          mode: bevy::audio::PlaybackMode::Once,
          volume: bevy::audio::Volume::default(),
          ..default()
        },
        // FadeIn,
      ));

    }
  }
}

// prettier-ignore
fn handle_drag(
  mut ev_m_motion: EventReader<bevy::input::mouse::MouseMotion>,
  mut query_camera: Query<&mut Transform, (With<PlayerCameraMarker>, Without<PlayerMarker>)>,
  mut player: Query<&mut Transform, (With<PlayerMarker>, Without<PlayerCameraMarker>)>
) {
  // if let Some(is_left_m_btn_down) = get_global_state() {
  //   if !is_left_m_btn_down { return; }
  // }

  // let mut transform = query_camera.single_mut();
  let mut trans_cam_parent = player.single_mut();
  let mut trans_cam = query_camera.single_mut();
  for ev_m in ev_m_motion.read() {
    // dbgln!("Mouse drag: X: {} px, Y: {} px", ev_m.delta.x, ev_m.delta.y);
    trans_cam_parent.rotate_local_y((ev_m.delta.x / 500.0) * 2.0 * -1.0);
    // trans_cam_parent.rotate_local_x((ev_m.delta.y / 1000.0) * 1.0);
    // trans_cam_parent.rotate_local_z((ev_m.delta.y / 1000.0) * 1.0);
    trans_cam.rotate_local_x((ev_m.delta.y / 1000.0) * -1.0);
    // trans_cam.rotate_local_z((ev_m.delta.y / 1000.0) * 1.0);
  }

}

fn keyboard_events(mut evr_kbd: EventReader<KeyboardInput>) {
  // key_code: KeyA, logical_key: Character("q")
  // key_code: KeyW, logical_key: Character("z")
  // key_code: KeyD, logical_key: Character("d")
  // key_code: KeyS, logical_key: Character("s")

  for ev in evr_kbd.read() {
    match ev.state {
      ButtonState::Pressed => {
        // dbgln!("Key press: {:?} {:?}", ev.key_code, ev.logical_key);
      }
      ButtonState::Released => {
        // dbgln!("Key release: {:?} {:?}", ev.key_code, ev.logical_key);
      }
    }
  }
}

// fn update_scroll_position(
//   mut mw_evt: EventReader<MouseWheel>,
//   mut query_camera: Query<&mut Transform, With<PlayerCameraMarker>>
// ) {
//   let transform = query_camera.single_mut();

//   for mouse_wheel_event in mw_evt.read() {
//     let (dx, dy) = match mouse_wheel_event.unit {
//       // MouseScrollUnit::Line => (mouse_wheel_event.x, mouse_wheel_event.y),
//       // MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
//       MouseScrollUnit::Line => (mouse_wheel_event.x, mouse_wheel_event.y),
//       MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
//     };

//     dbgln!("Mouse wheel: X: {}, Y: {}", dx, dy);

//     // transform.translation -= Vec3::new(0.0, dy / 1.0, 0.0);

//     // if kb_evt.pressed(KeyCode::ControlLeft) || kb_evt.pressed(KeyCode::ControlRight) {
//     //     std::mem::swap(&mut dx, &mut dy);
//     // }
//   }
// }

// fn zoom_perspective(mut query_camera: Query<&mut Projection, With<PlayerCameraMarker>>) {
//   // assume perspective. do nothing if orthographic.
//   let Projection::Perspective(persp) = query_camera.single_mut().into_inner() else {
//     return;
//   };
//   persp.fov /= 1.25; // zoom in
//   persp.fov *= 1.25; // zoom out
// }

// fn debug_cam_position(mut query_camera: Query<&mut Transform, With<PlayerCameraMarker>>) {
//   let mut transform = query_camera.single_mut();

//   transform.rotate_local_y(0.01);

//   match transform {
//     Transform::Perspective(persp) => {
//       // we have a perspective projection
//     }
//     Transform::Orthographic(ortho) => {
//       // we have an orthographic projection
//     }
//   }
// }

// static mut X: i32 = 0;
// fn debug_transform(query_camera: Query<&Transform, With<PlayerCameraMarker>>) {
//   unsafe {
//     X += 1;
//     if X % 100 == 0 {
//       let transform = query_camera.single();
//       dbgln!(
//         "cam: (:x, :y, :z) = ({}, {}, {})",
//         transform.translation.x,
//         transform.translation.y,
//         transform.translation.z
//       );
//     }
//   }
// }

// prettier-ignore
fn test_cam_update(
  mut query_camera: Query<&mut Transform, (With<PlayerCameraMarker>)>
) {

  // let mut trans_cam = query_camera.single_mut();
  // trans_cam.look_at(trans_sphere.translation, Vec3::Y);
  // trans_cam.translation.x = trans_sphere.translation.x + 5.0;
  // trans_cam.translation.y = trans_sphere.translation.y + 5.0;
  // trans_cam.translation.z = trans_sphere.translation.z + 5.0;


}
