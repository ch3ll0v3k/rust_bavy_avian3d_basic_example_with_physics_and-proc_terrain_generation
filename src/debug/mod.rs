use avian3d::prelude::*;
use bevy::color::palettes::tailwind::*;
use bevy::diagnostic::Diagnostic;
use bevy::diagnostic::DiagnosticPath;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::RegisterDiagnostic;
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;
use bevy::pbr::wireframe::WireframeConfig;
use bevy::pbr::wireframe::{ Wireframe, WireframePlugin };
use bevy::prelude::*;
use bevy_diagnostic::LogDiagnosticsPlugin;

use crate::markers::m_bevy::AnyObject;

pub const ALLOWED_DEBUG_PHYSICS: bool = !true;
pub const ALLOWED_DEBUG_ENGINE: bool = true;
pub const ALLOWED_DEBUG_FPS: bool = true;

const SYSTEM_ITERATION_COUNT: DiagnosticPath = DiagnosticPath::const_new("system_iteration_count");

// prettier-ignore
pub fn is_allowed_debug_physics() -> bool { ALLOWED_DEBUG_PHYSICS }
// prettier-ignore
pub fn is_allowed_debug_engine() -> bool { ALLOWED_DEBUG_ENGINE }
// prettier-ignore
pub fn is_allowed_debug_fps() -> bool { ALLOWED_DEBUG_FPS }

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, startup)
      .add_plugins((
        WireframePlugin,
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
      ))
      // // Wireframes can be configured with this resource. This can be changed at runtime.
      // .insert_resource(WireframeConfig {
      //   // The global wireframe config enables drawing of wireframes on every mesh,
      //   // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
      //   // regardless of the global configuration.
      //   global: false,
      //   default_color: Color::from(GREEN_100),
      // })

      .register_diagnostic(Diagnostic::new(SYSTEM_ITERATION_COUNT).with_suffix(" iterations"))
      .add_systems(Update, update)
      .add_systems(Update, toggle_wireframe)
      .add_systems(Update, get_fps);
  }
}

fn startup(
  mut commands: Commands
  //mut meshes: ResMut<Assets<Mesh>>
) {
  // prettier-ignore
  if !ALLOWED_DEBUG_FPS { return; }

  commands.spawn((Text::new("FPS: 0"), TextColor::from(RED_500), NameTextMarker));
}

fn update() {
  // for (entity, _cube_marker) in query.iter() {
  //     let mut position = cube_positions.0;
  //     position.x += time.delta_seconds();
  //     cube_positions.0 = position;
  //     commands.entity(entity).insert(position);
  // }
}

#[derive(Component, Debug, PartialEq, Eq)]
struct NameTextMarker;

// #[derive(Component, Debug, PartialEq, Eq)]
// struct NameSpanMarker;

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

fn get_fps(
  mut names: Query<&mut Text, With<NameTextMarker>>,
  diagnostics: Res<DiagnosticsStore>,
  fixed_time: Res<Time<Fixed>>
) {
  // prettier-ignore
  if !ALLOWED_DEBUG_FPS { return; }

  // let dt_f: f64 = fixed_time.delta_secs_f64();
  // let dt_dur: std::time::Duration = fixed_time.delta();
  if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
    let mut writer = names.single_mut();
    if let Some(raw) = fps.smoothed() {
      let s = format!("FPS: {:.2}", raw);
      // dbg!("{:?}", s);
      writer.0 = s.to_string();
      // *writer.text(text, 4) = format!("{raw:.2}");
    }
    // if let Some(raw) = fps.value() {
    //     let s = format!("{:.2}", raw);
    //     // dbg!("{:?}", s);
    //     writer.0 = s.to_string();
    //     // *writer.text(text, 4) = format!("{raw:.2}");
    // }
  }
}

fn toggle_wireframe(
  mut commands: Commands,
  all_wireframes: Query<Entity, (With<AnyObject>, With<Wireframe>)>,
  all_no_wireframe: Query<Entity, (With<AnyObject>, Without<Wireframe>)>,
  input: Res<ButtonInput<KeyCode>>
) {
  // prettier-ignore
  if !ALLOWED_DEBUG_ENGINE { return; }

  if input.just_pressed(KeyCode::KeyL) {
    for object in &all_no_wireframe {
      commands.entity(object).insert(Wireframe);
    }
    for object in &all_wireframes {
      commands.entity(object).remove::<Wireframe>();
    }
  }
}
