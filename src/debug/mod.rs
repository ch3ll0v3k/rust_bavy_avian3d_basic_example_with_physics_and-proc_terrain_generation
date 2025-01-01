// use std::time::Instant;

use std::ops::{ Add, Sub };
use std::time::Duration;

use instant::Instant;

// use avian3d::parry::na::Transform;
// use avian3d::prelude::*;
use avian3d::prelude::DebugRender;

use bevy::app::{ App, FixedUpdate, Plugin, Startup, Update };
use bevy::asset::{ AssetServer, Handle };
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
  Commands,
  Component,
  Entity,
  IntoSystemConfigs,
  KeyCode,
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
use bevy::time::{ Fixed, Time, Virtual };
use bevy::input::{ ButtonInput, common_conditions::input_just_pressed };
use crate::asset_loader::font_cache::{ FontCache, cache_load_font };

use crate::dbgln;

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
const MEASURE_AVG_FPS_EACH: u32 = 60;

#[derive(Component, Debug, PartialEq, Eq)]
struct FpsTextMarker;

#[derive(Component, Debug, PartialEq, Eq)]
struct FrameCounterTextMarker;

#[derive(Component, Debug, PartialEq, Eq)]
struct FrameTimetTextMarker;

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
      .add_systems(FixedUpdate, update)
      .add_systems(FixedUpdate, toggle_wireframe.run_if(input_just_pressed(KeyCode::KeyL)))
      .add_systems(FixedUpdate, (
          update_fps,
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
  mut commands: Commands
) {
  // prettier-ignore
  if !ALLOWED_DEBUG_FPS { return; }

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
    Text::new("Frame C: 0"),
    TextColor::from(BLACK),
    FrameCounterTextMarker,
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
    FrameTimetTextMarker,
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
fn calculate_real_fps_and_throttle(
  mut mut_frame_limiter: ResMut<FrameLimiter>,
  mut text: Query<&mut Text, With<FrameTimetTextMarker>>,
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

  // let raw_delta: f64 = elapsed.as_secs_f64() * 1.50;

  if !ALLOWED_DEBUG_FPS { return; }

  let raw_delta: f64 = elapsed.as_secs_f64() * 1.5;

  if mut_frame_limiter.avg_index >= MEASURE_AVG_FPS_EACH {
    let sum: f64 = mut_frame_limiter.avg.iter().sum::<f64>();
    let calculated_fps: f64 = sum / MEASURE_AVG_FPS_EACH as f64 * 100000.0 / 2.0;
    // let calculated_fps: f64 = FIXED_PFS as f64 / raw_delta / 100.0;
    // println!("sum: {}, calculated_fps: {}, elapsed: {}", sum, calculated_fps, raw_delta);
    let mut writer = text.single_mut();
    let s: String = format!("FPS (REAL): {:.2}", calculated_fps);
    writer.0 = s.to_string();
    mut_frame_limiter.avg_index = 0;
  }else{
    let index = mut_frame_limiter.avg_index as usize;
    mut_frame_limiter.avg[index] = raw_delta;
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
//   mut text: Query<&mut Text, With<FrameCounterTextMarker>>,
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
