// use std::time::Instant;

use std::ops::{ Add, Sub };
use std::time::Duration;

use bevy::pbr::{ MeshMaterial3d, StandardMaterial };
use instant::Instant;

// use avian3d::parry::na::Transform;
// use avian3d::prelude::*;
use avian3d::prelude::{
  AngularVelocity,
  CoefficientCombine,
  Collider,
  CollisionMargin,
  DebugRender,
  ExternalImpulse,
  GravityScale,
  LinearVelocity,
  Mass,
  Restitution,
  RigidBody,
};

use bevy::app::{ App, FixedUpdate, Plugin, Startup, Update };
use bevy::asset::{ AssetServer, Assets, Handle };
use bevy::color::palettes::css::*;
use bevy::color::{ Color, palettes::tailwind::* };

use bevy::ui::{ Node, PositionType, Val };
use bevy::utils::default;
use bevy_diagnostic::LogDiagnosticsPlugin;

use bevy::diagnostic::{
  Diagnostic,
  DiagnosticPath,
  DiagnosticsStore,
  EntityCountDiagnosticsPlugin,
  FrameTimeDiagnosticsPlugin,
  RegisterDiagnostic,
  SystemInformationDiagnosticsPlugin,
};

// use bevy::prelude::*;
use bevy::prelude::{
  in_state,
  Capsule3d,
  Commands,
  Component,
  Cuboid,
  Entity,
  IntoSystemConfigs,
  KeyCode,
  Mesh,
  Mesh3d,
  Query,
  Res,
  ResMut,
  Resource,
  Text,
  Transform,
  With,
  Without,
};

use bevy::math::Vec3;
use bevy::pbr::wireframe::{ Wireframe, WireframePlugin, WireframeConfig };
use bevy::text::{ Font, TextColor, TextFont };
use bevy::time::{ Fixed, Real, Time, Virtual };
use bevy::input::{ ButtonInput, common_conditions::input_just_pressed };
use crate::asset_loader::font_cache::{ FontCache, cache_load_font };

use crate::camera::CameraParentMarker;
use crate::{ dbgln, PhysicsDynamicObjectFloatable, COLLISION_MARGIN, GRAVITY };

use crate::markers::m_bevy::AnyObject;
use crate::state::MGameState;
use crate::sys_paths::font::EFontPaths;

pub const ALLOWED_DEBUG_PHYSICS: bool = !true;
pub const ALLOWED_DEBUG_ENGINE: bool = true;
pub const ALLOWED_DEBUG_FPS: bool = true;
pub const IS_WIREFRAME_DEFAULT_ON: bool = false;

const FPS_COUNTER_DIAG_: DiagnosticPath = DiagnosticPath::const_new("fps_counter");
const FIXED_PFS: f64 = 60.0;
const FRAMERATE_LIMIT: f64 = 1.0 / FIXED_PFS;
const MEASURE_AVG_FPS_EACH: u32 = 15;

#[derive(Component, Debug, PartialEq, Eq)]
struct FpsTextMarker;

#[derive(Component, Debug, PartialEq, Eq)]
struct PlayerYPosTextMarker;

#[derive(Component, Debug, PartialEq, Eq)]
struct FrameTimeTextMarker;

#[derive(Resource, Debug, PartialEq, Eq)]
struct IWireframeOn {
  is_on: bool,
}

#[derive(Resource)]
struct FrameLimiter {
  last_frame: Instant,
  frame_duration: Duration,
  avg_index: u32,
  avg: [f64; MEASURE_AVG_FPS_EACH as usize],
}

pub struct DebugPlugin;

// prettier-ignore
impl Plugin for DebugPlugin {
  fn build(&self, app: &mut App) {

    app
      // Wireframes can be configured with this resource. This can be changed at runtime.
      // .insert_resource(WireframeConfig {
      //   // The global wireframe config enables drawing of wireframes on every mesh,
      //   // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
      //   // regardless of the global configuration.
      //   global: false,
      //   default_color: Color::from(GREEN_100),
      // })
      .insert_resource(IWireframeOn { is_on: IS_WIREFRAME_DEFAULT_ON })
      .insert_resource(Time::<Fixed>::from_hz(FIXED_PFS))
      .insert_resource(FrameLimiter {
        last_frame: Instant::now(),
        frame_duration: Duration::from_secs_f64(FRAMERATE_LIMIT), // Target 60 FPS
        avg_index: 0,
        avg: [0.0; MEASURE_AVG_FPS_EACH as usize],
      })

    .add_systems(Startup, startup)
      .add_plugins((
        WireframePlugin,
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
      ))
      .register_diagnostic(Diagnostic::new(FPS_COUNTER_DIAG_)/*.with_suffix("can-be-anything")*/)
      .add_systems(FixedUpdate, (
        update
      ))
      .add_systems(FixedUpdate, (
        toggle_wireframe, 
      ).run_if(input_just_pressed(KeyCode::KeyL)))
      .add_systems(FixedUpdate, (
          update_fps,
          test_floating_items,
          show_player_y_pos,
        ).run_if(in_state(MGameState::Running))
      )
      .add_systems(Update, (
          calculate_real_fps_and_throttle,
        ).run_if(in_state(MGameState::Running))
      );

  }
}

// prettier-ignore
fn startup(
  mut res_mut_font_cache: Option<ResMut</*res_mut_font_cache::*/ FontCache>>,
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {
  // prettier-ignore
  if !ALLOWED_DEBUG_FPS { return; }

  let items = 5;
  let size = 5;
  let offset_x = 100;
  let offset_y = 250;
  let offset_z = 200; 
  let spread_xyz = 10; 

  for _y in 0..items {
    for _x in 0..items {
      for _z in 0..items {

        let y = (_y * size + _y * spread_xyz + offset_y) as f32;
        let x = (_x * size + _x * spread_xyz - offset_x) as f32;
        let z = (_z * size + _z * spread_xyz - offset_z) as f32;

        commands.spawn((
          RigidBody::Dynamic,
          CollisionMargin(COLLISION_MARGIN * 1.0),
          Collider::cuboid(size as f32, size as f32, size as f32),
          Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombine::Min,
          },
          Transform::from_translation(Vec3::new(x, y, z)),
          Mesh3d(meshes.add(Cuboid::new(size as f32, size as f32, size as f32))),
          MeshMaterial3d(materials.add(Color::srgb_u8(127, 255, 0))),
          Mass(100.0),
          // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
          // MaxLinearSpeed(5.0),
          PhysicsDynamicObjectFloatable,
        ));

      } 
    } 
  } 

  let font_hashmap: &mut ResMut<FontCache> = res_mut_font_cache.as_mut().unwrap();

  // let font_path = EFontPaths::QuartzoMain.as_str();
  // let font_path = EFontPaths::Bigpartyo2GreenMain.as_str();
  // let font_path = EFontPaths::LoveYouBlackSeeTrough.as_str();
  let font_path = EFontPaths::LoveYouBlackSolid.as_str();

  let font_handler: Handle<Font> = cache_load_font(font_hashmap, &asset_server, font_path, false);

  commands.spawn((
    Text::new("FPS: 0"),
    TextColor::from(BLACK),
    FpsTextMarker,
    TextFont {
      font: font_handler.clone(),
      font_size: 22.0,
      ..default()
    },
    Node {
      position_type: PositionType::Absolute,
      top: Val::Px(20.0),
      left: Val::Vw(1.5),
      ..default()
    },
  ));

  // #[cfg(not(feature = "default_font"))]
  commands.spawn((
    Text::new("P-Y (abs): 0"),
    TextColor::from(BLACK),
    PlayerYPosTextMarker,
    TextFont {
      font: font_handler.clone(),
      font_size: 22.0,
      ..default()
    },
    Node {
      position_type: PositionType::Absolute,
      // top: Val::Vh(4.5),
      top: Val::Px(50.0),
      left: Val::Vw(1.5),
      ..default()
    },
  ));

  commands.spawn((
    Text::new("Frame T: 0"),
    TextColor::from(BLACK),
    FrameTimeTextMarker,
    TextFont {
      font: font_handler.clone(),
      font_size: 22.0,
      ..default()
    },
    Node {
      position_type: PositionType::Absolute,
      top: Val::Px(80.0),
      left: Val::Vw(1.5),
      ..default()
    },
  ));
}

// prettier-ignore
fn update() {
  // for (entity, _cube_marker) in query.iter() {
  //   let mut position = cube_positions.0;
  //   position.x += time.delta_seconds();
  //   cube_positions.0 = position;
  //   commands.entity(entity).insert(position);
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
fn test_floating_items(
  mut commands: Commands,
  // mut q_lin_velocity: Query<&mut LinearVelocity, With<PhysicsDynamicObjectFloatable>>,
  mut q_selector: Query<
    (Entity, &mut RigidBody, &mut Transform, &mut LinearVelocity, &mut AngularVelocity),
    With<PhysicsDynamicObjectFloatable>
  >
) {
  let extara_down = -10.0;
  let f = -10.0 - 2.0 - extara_down;
  let df = 1000.0;
  let mut impulse3 = Vec3::new(0.0, 0.0, 0.0);
  let mut apply_force = false;
  let mut is_in_water = false;

  for (
    entity, 
    body, 
    mut transform, 
    mut lin_vel,
    mut ang_vel
  ) in q_selector.iter_mut() {

    
    if transform.translation.y < -f {
      // dbgln!("transform.translation.y: y {:.4}", transform.translation.y);

      is_in_water = true;

      if transform.translation.y < -2.0 {
        commands.entity(entity).insert(GravityScale(0.0));
        // commands.entity(entity).insert(GravityScale(0.06));
        // commands.entity(entity).insert(GravityScale(0.1));

        apply_force = true;
        let diff = transform.translation.y; //  - f;
        let abs_diff = (if diff < -df { -df } else { diff }).abs();
        let impulse = (abs_diff / df) * 20.0;
        let force = (abs_diff / df) * 10.0;
        let inverse = (diff / df).abs();

        // dbgln!("diff: y {:.4}", diff);

        // let mut vel = q_lin_velocity.single_mut();
        lin_vel.0 *= 0.98;
        ang_vel.0 *= 0.9975;

        // ang_vel.0 = ang_vel.0.clamp_length(-0.05, 0.05);

        // commands
        //   .entity(entity)
        //   .insert(LinearVelocity(Vec3::ZERO))
        //   .insert(AngularVelocity(Vec3::ZERO));

        impulse3.y = GRAVITY * impulse;
        // impulse3.y = GRAVITY * force;

        // if transform.translation.y < -8.0 {
        //   transform.translation.y += inverse * 100.0;
        //   // impulse3.y *= 2.0;
        // }
      } else {
        commands.entity(entity).insert(GravityScale(1.0));
      }
    } else {
      commands.entity(entity).insert(GravityScale(1.0));
    }

    let impulse = get_external_impulse(impulse3, false);
    commands.entity(entity).insert((RigidBody::Dynamic, impulse));
  }
}

// prettier-ignore
pub fn is_allowed_debug_physics() -> bool { ALLOWED_DEBUG_PHYSICS }
// prettier-ignore
pub fn is_allowed_debug_engine() -> bool { ALLOWED_DEBUG_ENGINE }
// prettier-ignore
pub fn is_allowed_debug_fps() -> bool { ALLOWED_DEBUG_FPS }

// prettier-ignore
pub fn get_defaul_physic_debug_params() -> DebugRender {
  if ALLOWED_DEBUG_PHYSICS {
    DebugRender::default()
      .with_collider_color(Color::srgb(1.0, 255.0, 1.0))
      .with_axes(Vec3::new(2.0, 2.0, 2.0))
      .with_aabb_color(Color::srgb(255.0, 0.0, 0.0))
  } else {
    DebugRender::none()
  }
}

// prettier-ignore
fn update_fps(
  mut text: Query<&mut Text, With<FpsTextMarker>>,
  diagnostics: Res<DiagnosticsStore>,
  fixed_time: Res<Time<Fixed>>
) {
  // prettier-ignore
  if !ALLOWED_DEBUG_FPS { return; }

  if let Some(diag_type) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
    let mut writer = text.single_mut();
    if let Some(raw) = diag_type.smoothed() {
      let s: String = format!("FPS (FIXED): {:.2}", raw);
      writer.0 = s.to_string();
    }
  }
}

// prettier-ignore
fn show_player_y_pos(
  mut text: Query<&mut Text, With<PlayerYPosTextMarker>>,
  q_camera_parent: Query<(& Transform), (With<CameraParentMarker>)>
) {

  // prettier-ignore
  if !ALLOWED_DEBUG_FPS { return; }
  
  let trans = q_camera_parent.single();
  let y = trans.translation.y;
  let mut writer = text.single_mut();
  let s: String = format!("P-Y (abs): {:.2}", y);
  writer.0 = s.to_string();
}

// prettier-ignore
fn calculate_real_fps_and_throttle(
  mut mut_frame_limiter: ResMut<FrameLimiter>,
  mut text: Query<&mut Text, With<FrameTimeTextMarker>>,
  fixed_time: Res<Time<Fixed>>,
  real_time: Res<Time<Real>>,
  virt_time: Res<Time<Virtual>>
) {

  // prettier-ignore
  let now: std::time::Instant = Instant::now();
  let elapsed: Duration = now.duration_since(mut_frame_limiter.last_frame);

  // std::thread::sleep(Duration::from_millis(40));

  if elapsed < mut_frame_limiter.frame_duration {
    let to_sleep = mut_frame_limiter.frame_duration - elapsed;
    // to_sleep = to_sleep.add(Duration::from_micros(5250));
    std::thread::sleep(to_sleep);
  }

  mut_frame_limiter.last_frame = Instant::now();

  if !ALLOWED_DEBUG_FPS { return; }

  let elapsed: f64 = elapsed.as_secs_f64();
  let fixed: f64 = fixed_time.delta_secs_f64();
  let real: f64 = real_time.delta_secs_f64();
  let virt: f64 = virt_time.delta_secs_f64();
  // println!("elapsed: {:.5}, fixed: {:.5}, real: {:.5}, virt: {:.5}", elapsed, fixed, real, virt);

  let elapsed_fps = (1000.0 / (elapsed * 1000.0));
  let fixed_fps = (1000.0 / (fixed * 1000.0));
  let real_fps = (1000.0 / (real * 1000.0));
  let virt_fps = (1000.0 / (virt * 1000.0));
  // println!("elapsed: {:.2}, fixed: {:.2}, real: {:.2}, virt: {:.2}", elapsed_fps, fixed_fps, real_fps, virt_fps);

  let fps_to_use = elapsed_fps;

  if mut_frame_limiter.avg_index >= MEASURE_AVG_FPS_EACH {
    let mut writer = text.single_mut();
    let s: String = format!("FPS (REAL): {:.2}", fps_to_use);
    writer.0 = s.to_string();
    mut_frame_limiter.avg_index = 0;
  }else{
    let index = mut_frame_limiter.avg_index as usize;
    mut_frame_limiter.avg[index] = fps_to_use;
    mut_frame_limiter.avg_index += 1;
  }
  
}

// prettier-ignore
fn toggle_wireframe(
  mut is_wireframe_on: ResMut<IWireframeOn>,
  mut commands: Commands,
  all_allowed_entities: Query<Entity, With<AnyObject>>,
  all_wireframes: Query<Entity, (With<AnyObject>, With<Wireframe>)>,
  all_no_wireframe: Query<Entity, (With<AnyObject>, Without<Wireframe>)>,
  input: Res<ButtonInput<KeyCode>>
) {
  // prettier-ignore
  if !ALLOWED_DEBUG_ENGINE { return; }

  is_wireframe_on.is_on = !is_wireframe_on.is_on;
  dbgln!("is_wireframe_on: {}", is_wireframe_on.is_on);

  if is_wireframe_on.is_on {
    for entity in &all_allowed_entities {
      commands.entity(entity).insert(Wireframe);
    }
  } else {
    for entity in &all_allowed_entities {
      commands.entity(entity).remove::<Wireframe>();
    }
  }

  // if input.just_pressed(KeyCode::KeyL) {
  //   for entity in &all_no_wireframe {
  //     commands.entity(entity).insert(Wireframe);
  //   }
  //   for entity in &all_wireframes {
  //     commands.entity(entity).remove::<Wireframe>();
  //   }
  // }
}

// // prettier-ignore
// fn update_frame_couter(
//   mut text: Query<&mut Text, With<PlayerYPosTextMarker>>,
//   diagnostics: Res<DiagnosticsStore>,
//   fixed_time: Res<Time<Fixed>>
// ) {
//   // prettier-ignore
//   if !ALLOWED_DEBUG_FPS { return; }

//   if let Some(diag_type) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT) {
//     let mut writer = text.single_mut();
//     if let Some(raw) = diag_type.smoothed() {
//       let s: String = format!("FPS: {:.2}", raw);
//       writer.0 = s.to_string();
//       // *writer.text(text, 4) = format!("{raw:.2}");
//     }
//   }
// }
