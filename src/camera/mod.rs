use std::{ borrow::BorrowMut, fmt, sync::Mutex };
// use tracing::instrument;

// use avian3d::prelude::*;
// use bevy::prelude::*;

use avian3d::collision::contact_reporting::{ Collision, CollisionEnded, CollisionStarted };
use avian3d::prelude::*;

use avian3d::parry::na;
use avian3d::prelude::{
  AngularVelocity,
  CoefficientCombine,
  Collider,
  CollisionMargin,
  ExternalAngularImpulse,
  ExternalForce,
  ExternalImpulse,
  ExternalTorque,
  LinearVelocity,
  LockedAxes,
  Mass,
  MaxLinearSpeed,
  Restitution,
  RigidBody,
};
use bevy::text::cosmic_text::ttf_parser::Tag;
use bevy::{
  animation::transition,
  input::{
    common_conditions::{ input_just_pressed, input_just_released },
    keyboard::KeyboardInput,
    mouse::{ MouseButtonInput, MouseScrollUnit, MouseWheel },
    ButtonState,
  },
  math::VectorSpace,
  pbr::wireframe::Wireframe,
  prelude::*,
  render::camera::PhysicalCameraParameters,
};

use crate::asset_loader::audio_cache::{ cache_load_audio, AudioCache };
use crate::debug::ALLOWED_DEBUG_ENGINE;
use crate::state::MGameState;
use crate::sys_paths;
use crate::{
  debug::{ get_defaul_physic_debug_params, is_allowed_debug_engine },
  entities::with_children::MEntityBigSphere,
  lights::{ MPointLightFromMarker, MPointLightMarker, MPointLightToMarker },
  AnyObject,
  COLLISION_MARGIN,
};

use sys_paths::audio::EAudioPaths;
use sys_paths::image::EImagePaths;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct CameraMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct CameraParentMarker;

#[derive(Resource)]
struct CMaxLinearSpeed(f32);

pub struct CameraPlugin;

// static mut IS_LEFT_DOWW: Option<bool> = Some(false);
// static IS_LEFT_DOWW: Mutex<Option<bool>> = Mutex::new(Some(false));

#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

// prettier-ignore
impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, startup)

      // .add_systems(Update, on_pause.run_if((
      //     in_state(MGameState::Paused)
      //   )
      // ))
      .add_systems(Update, 
        (
          handle_bullet_out_of_allowed_area,
          update,
          zoom_on_scroll,
          // control_cam,
          keyboard_events,
          // handle_drag,
          cam_track_object,
          cam_track_object_origin,
          detect_bullet_collision,
        ).run_if(in_state(MGameState::Running))
      )
      .add_systems(Update, control_cam)
      .add_systems(Update, handle_drag)
      // .add_systems(Update, update)
      // .add_systems(Update, zoom_on_scroll)
      // .add_systems(Update, keyboard_events)
      // .add_systems(Update, cam_track_object)
      // .add_systems(Update, cam_track_object_origin)
      // .add_systems(Update, detect_bullet_collision)
      .add_systems(Update, 
        (
          handle_left_click
        )
          .run_if(in_state(MGameState::Running))
          .run_if(input_just_pressed(MouseButton::Left))

      )
      .add_systems(Update, 
        (
          mk_jump
        )
          .run_if(in_state(MGameState::Running))
          .run_if(input_just_pressed(KeyCode::Space))
      )
      // .add_systems(Update, ( 
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
      .add_systems(Update, constrain_linear_xz_speed)
      .insert_resource(CMaxLinearSpeed(5.0)) // Set max speed for x + z axes
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


    }
}

const BULLET_MIN_Y_ALLOWED: f32 = -10.0;
const MUL_POS: f32 = 5.0;
const POS: Vec3 = Vec3::new(-2.5 * MUL_POS, 30.5 * MUL_POS, 9.0 * MUL_POS);

fn update() {}

// constrain linear-speed
// >> https://chatgpt.com/c/676a596e-4fd4-8000-9c52-8e4661d5dc76

// better option then Name::new("some tag")
// >> https://chatgpt.com/c/6773dcd8-1cd4-8000-bd81-a2e2507b9f5f

fn test(commands: &mut Commands) -> Entity {
  let id = commands.spawn(RigidBody::Dynamic).id();
  id
}

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {
  let id = test(&mut commands);

  // commands.spawn((
  //   RigidBody::Dynamic,
  //   // Collider::sphere(1.65),
  //   Collider::sphere(2.0),
  //   CollisionMargin(COLLISION_MARGIN * 1.0),
  //   Transform::from_xyz(-20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::ZERO),
  //   Mesh3d(meshes.add(Sphere::new(2.0))),
  //   MeshMaterial3d(materials.add(Color::srgb_u8(255, 40, 40))),
  //   // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
  //   Mass(10.0),
  //   Name::new("player_bal_t"),
  //   get_defaul_physic_debug_params(),
  //   AnyObject,
  // ));

  commands
    .spawn((
      RigidBody::Dynamic,
      CollisionMargin(COLLISION_MARGIN * 1.0),
      Collider::capsule(2.0, 5.0),
      // Restitution::new(0.0),
      Restitution {
        coefficient: 0.0,
        combine_rule: CoefficientCombine::Min,
      },
      Transform::from_xyz(POS.x, POS.y, POS.z).looking_at(POS, Vec3::Y),
      Mesh3d(meshes.add(Capsule3d::new(2.0, 5.0))),
      MeshMaterial3d(materials.add(Color::srgb_u8(127, 255, 0))),
      Mass(10.0),
      LockedAxes::ROTATION_LOCKED,
      // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
      // MaxLinearSpeed(5.0),
      CameraParentMarker,
      Name::new("p_player_t"),
    ))
    .with_children(|children| {
      children.spawn((
        Name::new("p_player_cam_t"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0), // .looking_at(POS, Vec3::Y),
        CameraMarker,
      ));
    });
}

// fn accelerate_bodies(mut query: Query<(&mut LinearVelocity, &mut AngularVelocity)>) {
//   for (mut linear_velocity, mut angular_velocity) in query.iter_mut() {
//     linear_velocity.x += 0.05;
//     angular_velocity.z += 0.05;
//   }
// }

// prettier-ignore
fn cam_track_object(
  // query_big_sphere: Query<&Transform, (With<MEntityBigSphere>, Without<MPointLightMarker>)>,
  // mut query_point_light: Query<&mut Transform, (With<MPointLightMarker>, Without<MEntityBigSphere>)>,
  // mut query_camera: Query<&mut Transform, (With<CameraMarker>, Without<MPointLightMarker>, Without<MEntityBigSphere>)>
) {

  // return;

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
  mut query_camera: Query<&mut Projection, With<CameraMarker>>
) {
  let Projection::Perspective(persp) = query_camera.single_mut().into_inner() else {
    return;
  };

  for mouse_wheel_event in mw_evt.read() {
    let (_dx, _dy) = match mouse_wheel_event.unit {
      // MouseScrollUnit::Line => (mouse_wheel_event.x, mouse_wheel_event.y),
      // MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
      MouseScrollUnit::Line => (mouse_wheel_event.x, mouse_wheel_event.y),
      MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
    };

    // dbg!("Mouse wheel: X: {}, Y: {}", dx, dy);

    let dy = _dy * -1.0;
    let val: f32 = persp.fov + dy / 30.0;

    // dbg!("FOV: {}", val);

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
  // dbg!("Q pressed");
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
//     dbg!("Camera forward direction: {:?}", forward);
//   }
// }

// #[tracing::instrument]
// prettier-ignore
// fn abc(
//   mut commands: Commands,
//   mut query: Single<&mut CameraParentMarker, (With<CameraParentMarker>)>,
//   mut cam_parent_2: Single<&mut CameraParentMarker, With<CameraParentMarker>>
// ) {
//   let( mut cam_parent) = query.into_inner();
//   // cam_parent
//   let p = cam_parent.into_inner();
//   // commands.entity(cam_parent)
// }

// prettier-ignore
fn mk_jump(
  mut commands: Commands,
  // query: Query<(Entity, &RigidBody, &Transform), With<CameraParentMarker>>
  query: Query<(Entity, &RigidBody), With<CameraParentMarker>>
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
  //   dbg!("Entity apply_impulse_at_point: entity: {:?}, force: {:?}", entity, force);
  //   commands
  //     .entity(entity)
  //     // .insert((RigidBody::Dynamic, force));
  //     .insert((RigidBody::Dynamic, force));
  //     // .insert(Force::new(force, transform.translation));
  // }
}

// prettier-ignore
fn get_external_impulse(impulse3: Vec3, is_persistent: bool) -> ExternalImpulse {
  let mut force = ExternalImpulse::default();
  force
    .apply_impulse_at_point(
      impulse3, 
      Vec3::ZERO, 
      Vec3::ZERO)
    .with_persistence(is_persistent);

  force
}

// prettier-ignore
fn get_external_force(impulse3: Vec3, is_persistent: bool) -> ExternalForce {
  let mut force = ExternalForce::default();
  force
    .apply_force_at_point(
      impulse3, 
      Vec3::ZERO, 
      Vec3::ZERO)
    .with_persistence(is_persistent);

  force
}

fn constrain_linear_xz_speed(
  mut q_lin_velocity: Query<&mut LinearVelocity, With<CameraParentMarker>>,
  max_speed: Res<CMaxLinearSpeed>
) {
  // for mut velocity in q_lin_velocity.iter_mut() {
  //   let xz_speed = (velocity.x.powi(2) + velocity.z.powi(2)).sqrt();
  //   if xz_speed > max_speed.0 {
  //     let scale = max_speed.0 / xz_speed;
  //     velocity.x *= scale;
  //     velocity.z *= scale;
  //   }
  // }
}

fn control_cam(
  g_state: Res<State<MGameState>>,
  mut q_lin_velocity: Query<&mut LinearVelocity, With<CameraParentMarker>>,
  mut commands: Commands,
  mut mw_evt: EventReader<MouseWheel>,
  keys: Res<ButtonInput<KeyCode>>,
  mut q_camera: Query<&Transform, (With<CameraMarker>, Without<CameraParentMarker>)>,
  // mut q_camera_parent: Query<&mut Transform, (With<CameraParentMarker>, Without<CameraMarker>)>
  // mut q_camera_parent_2: Query<
  //   &mut Transform,
  //   (With<CameraParentMarker>, Without<CameraMarker>)
  // >
  mut q_camera_parent: Query<
    (Entity, &mut RigidBody, &mut Transform),
    (With<CameraParentMarker>, Without<CameraMarker>)
  >
  // query: Query<(Entity, &RigidBody, &Transform), With<CameraParentMarker>>
) {
  // if let Some(is_left_m_btn_down) = get_global_state() {
  //   if !is_left_m_btn_down { return; }
  // }

  if
    !keys.pressed(KeyCode::KeyW) &&
    !keys.pressed(KeyCode::KeyS) &&
    !keys.pressed(KeyCode::KeyA) &&
    !keys.pressed(KeyCode::KeyD) &&
    !keys.pressed(KeyCode::Space)
  {
    return;
  }

  let (entity, body, mut transform) = q_camera_parent.single_mut();

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

  let mut max_speed: f32 = 5.0;
  let mut running_speed: f32 = 10.0; // 1.0;
  let mut jump_force: f32 = 0.0;
  let use_physics = true;

  if keys.pressed(KeyCode::KeyQ) {
    // running_speed = 2.0;
    // max_speed *= 3.0;
    max_speed *= 30.0;
  }

  if keys.pressed(KeyCode::Space) {
    jump_force = 100.0;
  }

  let m_state = g_state.get();

  let is_paused = m_state == &MGameState::Paused;

  let force_scale_mul: f32 = 100.0 * running_speed;
  const FW_DIV_SCALE: f32 = 20.0;
  const LR_DIV_SCALE: f32 = 1.0;
  const BOOST_SPEED: f32 = 0.5;

  let fw = transform.forward();
  let mut x = fw.x / FW_DIV_SCALE;
  let mut y = fw.y / FW_DIV_SCALE;
  let mut z = fw.z / FW_DIV_SCALE;

  let mut impulse3 = Vec3::new(0.0, 0.0, 0.0);
  impulse3.y = jump_force;

  if use_physics && !is_paused {
    let impl_x: f32 = x;
    let impl_y: f32 = y;
    let impl_z: f32 = z;

    if keys.pressed(KeyCode::KeyW) {
      impulse3 += Vec3::new(x, y, z) * force_scale_mul * 20.0 * BOOST_SPEED;
    } else if keys.pressed(KeyCode::KeyS) {
      impulse3 += Vec3::new(x * -1.0, y * -1.0, z * -1.0) * 20.0 * force_scale_mul * BOOST_SPEED;
    }

    let right = transform.right();
    x = (x - right.x) / LR_DIV_SCALE;
    y = (y - right.y) / LR_DIV_SCALE;
    z = (z - right.z) / LR_DIV_SCALE;
    if keys.pressed(KeyCode::KeyA) {
      impulse3 += Vec3::new(x, y, z) * force_scale_mul * BOOST_SPEED;
    } else if keys.pressed(KeyCode::KeyD) {
      impulse3 += Vec3::new(x * -1.0, y * -1.0, z * -1.0) * force_scale_mul * BOOST_SPEED;
    }
  } else {
    // dbg!("Camera is paused");
    if keys.pressed(KeyCode::KeyW) {
      transform.translation.x += x * 10.0;
      transform.translation.y += y * 10.0;
      transform.translation.z += z * 10.0;
    } else if keys.pressed(KeyCode::KeyS) {
      transform.translation.x -= x * 10.0;
      transform.translation.y -= y * 10.0;
      transform.translation.z -= z * 10.0;
    }

    let right = transform.right();
    x = (x - right.x) / LR_DIV_SCALE;
    y = (y - right.y) / LR_DIV_SCALE;
    z = (z - right.z) / 20.0;
    if keys.pressed(KeyCode::KeyA) {
      transform.translation.x += x;
      transform.translation.y += y;
      transform.translation.z += z;
    } else if keys.pressed(KeyCode::KeyD) {
      transform.translation.x -= x;
      transform.translation.y -= y;
      transform.translation.z -= z;
    }
  }

  let force = get_external_impulse(impulse3, false);
  commands.entity(entity).insert((RigidBody::Dynamic, force));

  for mut velocity in q_lin_velocity.iter_mut() {
    let xz_speed = (velocity.x.powi(2) + velocity.z.powi(2)).sqrt();
    if xz_speed > max_speed {
      let scale = max_speed / xz_speed;
      velocity.x *= scale;
      velocity.z *= scale;
    }
  }
}

// fn process_bullets(
//   mut commands: Commands,
//   // query: Query<(Entity, &RigidBody, &Transform), With<CameraParentMarker>>
//   q_bullet: Query<(Entity, &RigidBody), With<BulletMarker>>
// ) {
//   for (entity, rb_bullet) in q_bullet.iter() {
//     if rb_bullet.is_dynamic() {
//       dbg!("Entity : entity: {:?}, bullet: {:?}", entity, rb_bullet);
//       commands.entity(entity).despawn();
//     }
//   }
// }

// fn detect_collisions(mut collision_events: EventReader<CollisionEvent>) {
//   for event in collision_events.iter() {
//     match event {
//       CollisionEvent::Started(entity1, entity2) => {
//         dbg!("Collision started between {:?} and {:?}", entity1, entity2);
//       }
//       CollisionEvent::Stopped(entity1, entity2) => {
//         dbg!("Collision stopped between {:?} and {:?}", entity1, entity2);
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
    //   dbg!(" end > : (type(1): {type_t_1}) collided (type(2): {type_t_2})");
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

      // dbg!(" start > : (type(1): {type_t_1}) collided (type(2): {type_t_2})");

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

      // dbg!(
      //   "(Entities (name: {type_t_1} => {}) and (name: {type_t_2} => {}) are colliding), (bodies: {:?} and {:?} ) is_sensor: {:?}, collision_started: {:?}",
      //   contacts.entity1,
      //   contacts.entity2,
      //   contacts.body_entity1,
      //   contacts.body_entity2,
      //   contacts.is_sensor,
      //   contacts.collision_started()
      // );
      // dbg!(
      //   "0: (type(1): {type_t_1}) collided (type(2): {type_t_2}) ({:?} => {:?})",
      //   contacts.body_entity1,
      //   contacts.body_entity2
      // );

      // return;

      if type_t_1 == "p_bullet_t" {
        dbg!(
          "0: (type(1): {type_t_1}) collided (type(2): {type_t_2}) ({} => {})",
          contacts.entity1,
          contacts.entity2
        );
        commands.entity(contacts.entity1).despawn();
      }

      if type_t_2 == "p_bullet_t" {
        dbg!(
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
  // query: Query<(Entity, &RigidBody, &Transform), With<CameraParentMarker>>
  q_bullets: Query<(Entity, &Transform), With<BulletMarker>>
) {
  for (entity, transform) in q_bullets.iter() {
    if transform.translation.y < BULLET_MIN_Y_ALLOWED {
      let bullet_t = q_name.get(entity).unwrap_or(&Name::new("unknown_t")).to_string();
      // dbg!("Bullet out of allowed area: {:?}", bullet_t);
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
  mut player: Query<&mut Transform, (With<CameraParentMarker>, Without<CameraMarker>)>,
  mut query_camera: Query<&mut Transform, (With<CameraMarker>, Without<CameraParentMarker>)>
) {
  for ev_b in ev_b_input.read() {
    if ev_b.button == MouseButton::Left {

      let mut transform_parent = player.single_mut();
      let mut transform = query_camera.single_mut();
      let vec3_parent = transform_parent.translation;
      let fw_parent = Vec3::from(transform_parent.forward());
      let up_child = Vec3::from(transform.up());
      let mut to = Vec3::new( fw_parent.x,  up_child.z * 1.5, fw_parent.z);
      // dbg!("to-vec-y: {} => up_child.z {}", transform.translation.y, up_child.z);
      
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
        // if ALLOWED_DEBUG_ENGINE { Wireframe } else { Wireframe::default() }, 
      );

      commands.spawn(handle_bullet);
      // commands.entity(object).insert(handle_bullet);
      // commands.entity(object).insert(Wireframe);

      // // dbg!("Right mouse button pressed");
      // for ev_m in ev_m_motion.read() {
      //   // dbg!("Mouse drag: X: {} px, Y: {} px", ev_m.delta.x, ev_m.delta.y);
      //   transform.rotate_local_y(ev_m.delta.x / 1000.0);
      //   transform.rotate_local_x((ev_m.delta.y / 1000.0) * 1.0);
      // }

      let audio_hashmap: &mut ResMut<AudioCache> = res_mut_audio_cache.as_mut().unwrap();

      let sound = cache_load_audio(
        audio_hashmap, 
        &asset_server, 
        EAudioPaths::PaintballShoot.as_str(),
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
  mut query_camera: Query<&mut Transform, (With<CameraMarker>, Without<CameraParentMarker>)>,
  mut player: Query<&mut Transform, (With<CameraParentMarker>, Without<CameraMarker>)>
) {
  // if let Some(is_left_m_btn_down) = get_global_state() {
  //   if !is_left_m_btn_down { return; }
  // }

  // let mut transform = query_camera.single_mut();
  let mut trans_cam_parent = player.single_mut();
  let mut trans_cam = query_camera.single_mut();
  for ev_m in ev_m_motion.read() {
    // dbg!("Mouse drag: X: {} px, Y: {} px", ev_m.delta.x, ev_m.delta.y);
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
        // dbg!("Key press: {:?} {:?}", ev.key_code, ev.logical_key);
      }
      ButtonState::Released => {
        // dbg!("Key release: {:?} {:?}", ev.key_code, ev.logical_key);
      }
    }
  }
}

// fn update_scroll_position(
//   mut mw_evt: EventReader<MouseWheel>,
//   mut query_camera: Query<&mut Transform, With<CameraMarker>>
// ) {
//   let transform = query_camera.single_mut();

//   for mouse_wheel_event in mw_evt.read() {
//     let (dx, dy) = match mouse_wheel_event.unit {
//       // MouseScrollUnit::Line => (mouse_wheel_event.x, mouse_wheel_event.y),
//       // MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
//       MouseScrollUnit::Line => (mouse_wheel_event.x, mouse_wheel_event.y),
//       MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
//     };

//     dbg!("Mouse wheel: X: {}, Y: {}", dx, dy);

//     // transform.translation -= Vec3::new(0.0, dy / 1.0, 0.0);

//     // if kb_evt.pressed(KeyCode::ControlLeft) || kb_evt.pressed(KeyCode::ControlRight) {
//     //     std::mem::swap(&mut dx, &mut dy);
//     // }
//   }
// }

// fn zoom_perspective(mut query_camera: Query<&mut Projection, With<CameraMarker>>) {
//   // assume perspective. do nothing if orthographic.
//   let Projection::Perspective(persp) = query_camera.single_mut().into_inner() else {
//     return;
//   };
//   persp.fov /= 1.25; // zoom in
//   persp.fov *= 1.25; // zoom out
// }

// fn debug_cam_position(mut query_camera: Query<&mut Transform, With<CameraMarker>>) {
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
// fn debug_transform(query_camera: Query<&Transform, With<CameraMarker>>) {
//   unsafe {
//     X += 1;
//     if X % 100 == 0 {
//       let transform = query_camera.single();
//       dbg!(
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
  mut query_camera: Query<&mut Transform, (With<CameraMarker>)>
) {

  // let mut trans_cam = query_camera.single_mut();
  // trans_cam.look_at(trans_sphere.translation, Vec3::Y);
  // trans_cam.translation.x = trans_sphere.translation.x + 5.0;
  // trans_cam.translation.y = trans_sphere.translation.y + 5.0;
  // trans_cam.translation.z = trans_sphere.translation.z + 5.0;


}
